use core::str;

use crate::size;

const BASE64_ALPHABET: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

const BASE64_GROUP_BYTES: u64 = 3;
const BASE64_GROUP_CHARACTERS: u64 = 4;
const PERCENT_ESCAPE_BYTES: u64 = 3;
const SEXTET_MASK: u32 = 0x3f;
const _: () = assert!(BASE64_ALPHABET.len() == 64, "base64 maps six bits onto 64 characters");

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Sextet {
    First,
    Fourth,
    Second,
    Third,
}

impl Sextet {
    const fn shift(self) -> u32 {
        match self {
            Self::First => 18,
            Self::Fourth => 0,
            Self::Second => 12,
            Self::Third => 6,
        }
    }
}

#[must_use]
pub(crate) fn base64(input: &[u8]) -> String {
    assert!(!input.is_empty(), "a credential pair is never empty");

    let group_count = size::of_slice(input).div_ceil(BASE64_GROUP_BYTES);
    let capacity = group_count.strict_mul(BASE64_GROUP_CHARACTERS);
    let mut encoded = String::with_capacity(size::offset(capacity));

    for chunk in input.chunks(3) {
        base64_group_push(&mut encoded, chunk);
    }

    assert_eq!(size::of_text(&encoded), capacity, "each group of three bytes is four characters");
    assert!(size::of_text(&encoded).is_multiple_of(4), "base64 is padded to whole groups");

    encoded
}

fn base64_character(group: u32, sextet_position: Sextet) -> char {
    assert!(group < 1_u32 << 24_u32, "a group packs three bytes");

    let shift = sextet_position.shift();

    assert!(shift.is_multiple_of(6), "a sextet starts on a multiple of six bits");

    let sextet = group.wrapping_shr(shift) & SEXTET_MASK;
    let position = size::offset(u64::from(sextet));

    BASE64_ALPHABET.get(position).map_or('A', |byte| char::from(*byte))
}

fn base64_group_push(encoded: &mut String, chunk: &[u8]) {
    assert!(!chunk.is_empty(), "a chunk carries at least one byte");
    assert!(size::of_slice(chunk) <= BASE64_GROUP_BYTES, "a chunk carries at most three bytes");

    let first = chunk.first().copied().unwrap_or(0);
    let second = chunk.get(1).copied().unwrap_or(0);
    let third = chunk.get(2).copied().unwrap_or(0);

    let group =
        u32::from(first).wrapping_shl(16) | u32::from(second).wrapping_shl(8) | u32::from(third);

    encoded.push(base64_character(group, Sextet::First));
    encoded.push(base64_character(group, Sextet::Second));

    if chunk.len() < 2 {
        encoded.push('=');
    } else {
        encoded.push(base64_character(group, Sextet::Third));
    }

    if chunk.len() < 3 {
        encoded.push('=');
    } else {
        encoded.push(base64_character(group, Sextet::Fourth));
    }
}

const fn byte_is_unreserved(byte: u8) -> bool {
    if byte.is_ascii_alphanumeric() {
        return true;
    }

    matches!(byte, b'-' | b'.' | b'_' | b'~')
}

fn hex_digit(nibble: u8) -> char {
    assert!(nibble <= 0x0f, "a hex digit encodes four bits");

    char::from_digit(u32::from(nibble), 16).unwrap_or('0').to_ascii_uppercase()
}

#[must_use]
pub(crate) fn percent(value: &str) -> String {
    assert!(!value.is_empty(), "an empty query term is never encoded");

    let bytes_max = size::of_text(value).strict_mul(PERCENT_ESCAPE_BYTES);
    let mut encoded = String::with_capacity(size::offset(bytes_max));

    for byte in value.bytes() {
        if byte_is_unreserved(byte) {
            encoded.push(char::from(byte));

            continue;
        }

        encoded.push('%');
        encoded.push(hex_digit(byte.wrapping_shr(4)));
        encoded.push(hex_digit(byte & 0x0f));
    }

    assert!(encoded.len() >= value.len(), "escaping never shortens a value");
    assert!(size::of_text(&encoded) <= bytes_max, "escaping at most triples a value");

    encoded
}

#[must_use]
pub(crate) fn percent_decode(text: &str) -> Vec<u8> {
    let bytes = text.as_bytes();
    let mut decoded: Vec<u8> = Vec::with_capacity(bytes.len());
    let mut position: u64 = 0;

    for _byte_index in 0..bytes.len() {
        let start = size::offset(position);

        let Some(byte) = bytes.get(start) else {
            break;
        };

        let digits = bytes.get(start.strict_add(1)..start.strict_add(3));
        let pair = digits.and_then(|found| str::from_utf8(found).ok());
        let escape = pair.filter(|_found| *byte == b'%');

        if let Some(found) = escape.and_then(|pair_text| u8::from_str_radix(pair_text, 16).ok()) {
            decoded.push(found);
            position = position.strict_add(3);
        } else {
            decoded.push(*byte);
            position = position.strict_add(1);
        }
    }

    assert!(decoded.len() <= bytes.len(), "decoding never lengthens a value");
    assert!(size::offset(position) >= decoded.len(), "each decoded byte spends an input byte");

    decoded
}

#[cfg(test)]
mod tests {
    use super::{base64, percent, percent_decode};

    #[test]
    fn base64_matches_known_vectors() {
        assert_eq!(base64(b"f"), "Zg==", "one byte pads twice");
        assert_eq!(base64(b"fo"), "Zm8=", "two bytes pad once");
        assert_eq!(base64(b"foo"), "Zm9v", "three bytes pad never");
        assert_eq!(base64(b"foob"), "Zm9vYg==", "a fourth byte starts a new group");
        assert_eq!(base64(b"\x00\xff\x80"), "AP+A", "every bit pattern maps");

        assert_eq!(
            base64(b"user@example.com:token123"),
            "dXNlckBleGFtcGxlLmNvbTp0b2tlbjEyMw==",
            "a credential pair encodes as the HTTP header expects",
        );
    }

    #[test]
    #[should_panic(expected = "a credential pair is never empty")]
    fn base64_refuses_an_empty_input() {
        let _encoded = base64(b"");
    }

    #[test]
    fn percent_decode_reverses_the_escapes_the_uri_standard_names() {
        assert_eq!(percent_decode("a%20b%2Fc"), b"a b/c", "two escapes");
        assert_eq!(percent_decode("%C3%A9"), "\u{e9}".as_bytes(), "two bytes of one character");
        assert_eq!(percent_decode("plain"), b"plain", "nothing to do");
        assert_eq!(percent_decode("100%"), b"100%", "a bare percent stays");
        assert_eq!(percent_decode("%zz"), b"%zz", "a bad escape stays");
    }

    #[test]
    fn percent_decode_undoes_percent_for_every_byte_a_text_can_hold() {
        for value in ["a b", "x/y?z=1&w", "\u{e9}\u{4e16}\u{1f600}", "100%", "%2F"] {
            assert_eq!(percent_decode(&percent(value)), value.as_bytes(), "{value} round trips");
        }
    }

    #[test]
    fn percent_escapes_everything_outside_the_unreserved_set() {
        assert_eq!(percent("a-b.c_d~e9"), "a-b.c_d~e9", "the unreserved set passes");
        assert_eq!(percent(" "), "%20", "a space escapes");
        assert_eq!(percent("&"), "%26", "an ampersand escapes");
        assert_eq!(percent("="), "%3D", "an equals sign escapes");
        assert_eq!(percent("/"), "%2F", "a slash escapes");
        assert_eq!(percent("\""), "%22", "a quote escapes");
        assert_eq!(percent("\u{e9}"), "%C3%A9", "each UTF-8 byte escapes");

        assert_eq!(
            percent("creator = \"5b10ac\""),
            "creator%20%3D%20%225b10ac%22",
            "a CQL term escapes whole",
        );
    }

    #[test]
    #[should_panic(expected = "an empty query term is never encoded")]
    fn percent_refuses_an_empty_input() {
        let _encoded = percent("");
    }
}
