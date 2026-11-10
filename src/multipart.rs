use crate::error::Error;
use crate::part::Part;

const BOUNDARY_ATTEMPT_COUNT_MAX: u64 = 16;
const FNV_OFFSET: u64 = 0xcbf2_9ce4_8422_2325;
const FNV_PRIME: u64 = 0x0000_0100_0000_01b3;
const LINE_END: &[u8] = b"\r\n";
const _: () = assert!(BOUNDARY_ATTEMPT_COUNT_MAX >= 1, "a body gets at least one boundary");

pub(crate) fn boundary(parts: &[Part<'_>]) -> Result<String, Error> {
    assert!(!parts.is_empty(), "a multipart body holds at least one part");

    let digest = parts_digest(parts);

    for attempt in 0..BOUNDARY_ATTEMPT_COUNT_MAX {
        let candidate = format!("confluence-{digest:016x}-{attempt}");

        if !parts.iter().any(|part| part_holds(part, &candidate)) {
            assert!(candidate.is_ascii(), "a boundary is plain ASCII");
            assert!(!candidate.is_empty(), "a boundary is never empty");

            return Ok(candidate);
        }
    }

    Err(Error::Input("no multipart boundary is absent from the parts".to_owned()))
}

#[must_use]
pub(crate) fn encode(parts: &[Part<'_>], boundary: &str) -> Vec<u8> {
    assert!(!parts.is_empty(), "a multipart body holds at least one part");
    assert!(!boundary.is_empty(), "a multipart body has a boundary");

    let mut body: Vec<u8> = Vec::new();

    for part in parts {
        part_push(&mut body, part, boundary);
    }

    body.extend_from_slice(b"--");
    body.extend_from_slice(boundary.as_bytes());
    body.extend_from_slice(b"--");
    body.extend_from_slice(LINE_END);

    assert!(body.starts_with(b"--"), "a body opens on a boundary");
    assert!(body.ends_with(b"--\r\n"), "a body closes on the final boundary");

    body
}

fn header_line(text: &str) -> String {
    let escaped = text.replace('\r', "%0D").replace('\n', "%0A");

    assert!(!escaped.contains('\r'), "a header value never breaks its line");
    assert!(!escaped.contains('\n'), "a header value never starts a new header");

    escaped
}

fn header_quoted(text: &str) -> String {
    let escaped = header_line(&text.replace('\\', "\\\\").replace('"', "\\\""));

    assert!(escaped.len() >= text.len(), "an escape only ever adds to a value");
    assert!(!escaped.contains('\n'), "a quoted header value stays on its line");

    escaped
}

fn part_holds(part: &Part<'_>, boundary: &str) -> bool {
    assert!(!boundary.is_empty(), "a boundary is never empty");
    assert!(!part.name.is_empty(), "a part is named");

    part.bytes.windows(boundary.len()).any(|window| window == boundary.as_bytes())
}

fn part_push(body: &mut Vec<u8>, part: &Part<'_>, boundary: &str) {
    assert!(!boundary.is_empty(), "a part follows a boundary");
    assert!(!part.name.is_empty(), "a part is named");

    let name = header_quoted(part.name);

    let disposition = part.file_name.map_or_else(
        || format!("Content-Disposition: form-data; name=\"{name}\""),
        |file_name| {
            let escaped = header_quoted(file_name);

            format!("Content-Disposition: form-data; name=\"{name}\"; filename=\"{escaped}\"")
        },
    );

    body.extend_from_slice(b"--");
    body.extend_from_slice(boundary.as_bytes());
    body.extend_from_slice(LINE_END);
    body.extend_from_slice(disposition.as_bytes());
    body.extend_from_slice(LINE_END);
    body.extend_from_slice(b"Content-Type: ");
    body.extend_from_slice(header_line(part.content_type).as_bytes());
    body.extend_from_slice(LINE_END);
    body.extend_from_slice(LINE_END);
    body.extend_from_slice(part.bytes);
    body.extend_from_slice(LINE_END);
}

fn parts_digest(parts: &[Part<'_>]) -> u64 {
    assert!(!parts.is_empty(), "a digest reads at least one part");

    let mut digest = FNV_OFFSET;

    for part in parts {
        for byte in part.name.bytes().chain(part.bytes.iter().copied()) {
            digest = (digest ^ u64::from(byte)).wrapping_mul(FNV_PRIME);
        }
    }

    digest
}

#[cfg(test)]
mod tests {
    use super::{boundary, encode, part_holds, parts_digest};
    use crate::part::Part;

    #[test]
    fn a_body_matches_the_wire_form_of_rfc_7578() {
        let parts = [file(b"hello")];
        let body = encode(&parts, "edge");

        let expected = "--edge\r\n\
            Content-Disposition: form-data; name=\"file\"; filename=\"notes.txt\"\r\n\
            Content-Type: text/plain\r\n\
            \r\n\
            hello\r\n\
            --edge--\r\n";

        assert_eq!(String::from_utf8(body).expect("plain text"), expected, "the bytes on the wire");
    }

    #[test]
    fn a_boundary_avoids_every_candidate_a_part_already_holds() {
        let digest = parts_digest(&[file(b"x")]);
        let first = format!("confluence-{digest:016x}-0");
        let held = [file(first.as_bytes())];
        let chosen = boundary(&held).expect("another candidate");

        assert!(part_holds(&held[0], &first), "the first candidate is taken");
        assert!(!part_holds(&held[0], &chosen), "the chosen one is absent");
    }

    #[test]
    fn a_digest_is_the_published_fnv_1a_of_its_name_and_bytes() {
        let part = Part { bytes: b"bar", content_type: "text/plain", file_name: None, name: "foo" };

        assert_eq!(parts_digest(&[part]), 0x8594_4171_f739_67e8, "the published vector of foobar");
    }

    #[test]
    fn a_digest_of_zero_is_an_ordinary_digest_rather_than_a_crash() {
        let bytes = [
            0x44_u8,
            0x20,
            0x82,
            0x3c,
            0x02,
            0x00,
            0x0e,
            0x01,
            0x05,
            0x05,
            0x05,
            0x02,
            0x1e,
            0x00,
            0xf8,
            0x01,
            0x08,
            0x04,
            0x01,
            0x01,
        ];

        let parts = [file(&bytes)];

        assert_eq!(parts_digest(&parts), 0, "these bytes land the digest on exactly zero");
        assert!(boundary(&parts).is_ok(), "and a boundary still comes out");
    }

    #[test]
    fn a_field_without_a_file_name_is_a_plain_form_field() {
        let parts = [Part {
            bytes: b"summary",
            content_type: "text/plain",
            file_name: None,
            name: "columns",
        }];

        let text = String::from_utf8(encode(&parts, "edge")).expect("plain text");

        assert!(text.contains("name=\"columns\"\r\n"), "the field is named");
        assert!(!text.contains("filename"), "a form field names no file");
    }

    #[test]
    fn a_name_that_tries_to_start_a_header_stays_inside_its_quotes() {
        let hostile = "a\"\r\nX-Injected: yes";

        let parts = [Part {
            bytes: b"x",
            content_type: "text/plain",
            file_name: Some(hostile),
            name: hostile,
        }];

        let text = String::from_utf8(encode(&parts, "edge")).expect("plain text");

        assert!(!text.contains("\r\nX-Injected"), "no header was injected");
        assert_eq!(text.matches("a\\\"").count(), 2, "each quote is escaped");
        assert!(text.contains("%0D%0AX-Injected"), "a line break cannot end the header");
    }

    #[test]
    fn a_quoted_value_decodes_back_to_what_was_named() {
        let named = "a \"quoted\" back\\slash caf\u{e9}.txt";

        let parts = [Part {
            bytes: b"x",
            content_type: "text/plain; charset=\"utf-8\"",
            file_name: Some(named),
            name: "file",
        }];

        let text = String::from_utf8(encode(&parts, "edge")).expect("plain text");
        let (_before, rest) = text.split_once("filename=\"").expect("the header names a file");

        assert_eq!(quoted_string(rest), named, "a reader recovers the exact name");
        assert!(text.contains("charset=\"utf-8\""), "a media type parameter keeps its quotes");
    }

    fn file(bytes: &[u8]) -> Part<'_> {
        Part { bytes, content_type: "text/plain", file_name: Some("notes.txt"), name: "file" }
    }

    fn quoted_string(text: &str) -> String {
        let mut found = String::new();
        let mut characters = text.chars();

        for _character_index in 0..text.len() {
            match characters.next() {
                Some('"') | None => break,
                Some('\\') => found.extend(characters.next()),
                Some(character) => found.push(character),
            }
        }

        assert!(found.len() <= text.len(), "a decoded value never outgrows what was read");

        found
    }
}
