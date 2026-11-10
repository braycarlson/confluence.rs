#[must_use]
pub(crate) fn of_offset(position: usize) -> u64 {
    assert!(
        u64::try_from(position).is_ok(),
        "an offset fits a count on every target this crate builds for",
    );

    let count = u64::try_from(position).unwrap_or(u64::MAX);

    assert_eq!(offset(count), position, "an offset becomes a count and comes back without loss");

    count
}

#[must_use]
pub(crate) fn of_slice<Item>(items: &[Item]) -> u64 {
    let count = of_offset(items.len());

    assert_eq!(count == 0, items.is_empty(), "a count is zero exactly when the slice is empty");

    count
}

#[must_use]
pub(crate) fn of_text(text: &str) -> u64 {
    let count = of_offset(text.len());

    assert_eq!(count == 0, text.is_empty(), "a byte count is zero exactly when the text is empty");

    count
}

#[must_use]
pub(crate) fn offset(count: u64) -> usize {
    assert!(usize::try_from(count).is_ok(), "a count fits an offset on the target it runs on");

    let position = usize::try_from(count).unwrap_or(usize::MAX);

    assert_eq!(position == 0, count == 0, "an offset is zero exactly when its count is");

    position
}

#[cfg(test)]
mod tests {
    use super::{of_offset, of_slice, of_text, offset};

    #[test]
    fn a_count_and_an_offset_round_trip_at_every_width_both_share() {
        for count in [0_u64, 1, 7, 4_096, u64::from(u32::MAX)] {
            assert_eq!(of_offset(offset(count)), count, "{count} survives the trip");

            assert_eq!(
                offset(count) == 0,
                count == 0,
                "{count} is zero exactly when its offset is",
            );
        }
    }

    #[test]
    fn a_slice_and_a_text_count_what_they_hold() {
        let empty: [u8; 0] = [];

        assert_eq!(of_slice(&empty), 0, "an empty slice counts nothing");
        assert_eq!(of_slice(&[1_u8, 2, 3]), 3, "three items count three");
        assert_eq!(of_text(""), 0, "an empty text counts nothing");
        assert_eq!(of_text("\u{e9}"), 2, "a text counts its bytes, not its characters");
    }
}
