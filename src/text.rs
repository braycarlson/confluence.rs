use crate::size;

pub(crate) const CODE_POINT_BYTES_MAX: u64 = 4;
const _: () = assert!(CODE_POINT_BYTES_MAX >= 1, "a code point takes at least one byte");

#[must_use]
pub(crate) fn boundary_floor(text: &str, byte_limit: u64) -> u64 {
    assert!(
        byte_limit <= size::of_text(text),
        "a boundary is looked for inside the text, never past its end",
    );

    let mut boundary = size::offset(byte_limit);

    for _step_index in 0..CODE_POINT_BYTES_MAX {
        if text.is_char_boundary(boundary) {
            break;
        }

        boundary = boundary.strict_sub(1);
    }

    let found = size::of_offset(boundary);

    assert!(
        text.is_char_boundary(boundary),
        "a code point is at most four bytes, so four steps back always reach a boundary",
    );

    assert!(found <= byte_limit, "a boundary floor never rises above its limit");

    assert!(
        found.strict_add(CODE_POINT_BYTES_MAX) > byte_limit,
        "a boundary floor sits within one code point of its limit",
    );

    found
}

#[must_use]
pub(crate) fn prefix(text: &str, byte_limit: u64) -> &str {
    assert!(
        byte_limit >= CODE_POINT_BYTES_MAX,
        "a budget below one code point could cut every character",
    );

    if size::of_text(text) <= byte_limit {
        return text;
    }

    let boundary = size::offset(boundary_floor(text, byte_limit));
    let cut = text.get(..boundary).unwrap_or_default();

    assert!(size::of_text(cut) <= byte_limit, "a prefix fits its budget");
    assert!(cut.len() < text.len(), "a prefix of an over-budget text is shorter");

    cut
}

#[cfg(test)]
mod tests {
    use super::{boundary_floor, prefix};

    #[test]
    fn a_boundary_at_the_end_of_the_string_is_the_string() {
        let text = "abcdef";

        assert_eq!(boundary_floor(text, 6), 6, "the end of a text is a boundary");
        assert_eq!(boundary_floor(text, 0), 0, "the start of a text is a boundary");
    }

    #[test]
    #[should_panic(expected = "a boundary is looked for inside the text")]
    fn a_boundary_past_the_end_is_a_programmer_error() {
        let _boundary = boundary_floor("abc", 4);
    }

    #[test]
    fn a_cut_lands_on_a_code_point_boundary_rather_than_inside_one() {
        let text = "\u{4e16}\u{754c}\u{3002}";

        assert_eq!(text.len(), 9, "three three-byte characters");
        assert_eq!(prefix(text, 4), "\u{4e16}", "four bytes hold one character");
        assert_eq!(prefix(text, 8), "\u{4e16}\u{754c}", "eight bytes hold two");
        assert_eq!(prefix(text, 9), text, "nine bytes hold all three");
    }

    #[test]
    fn a_string_inside_the_budget_passes_through_untouched() {
        assert_eq!(prefix("short", 64), "short", "a short text is its own prefix");
        assert_eq!(prefix("", 64), "", "an empty text stays empty");
    }
}
