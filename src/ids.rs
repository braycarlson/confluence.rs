use core::borrow::Borrow;
use core::fmt;

use crate::limits;
use crate::size;

const ATTACHMENT_PREFIX: &str = "att";
const SPACE_KEY_PERSONAL: char = '~';

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AccountID(String);

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct AttachmentID(String);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct BlogPostID(i64);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CommentID(i64);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct PageID(i64);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SpaceID(i64);

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct SpaceKey(String);

impl AccountID {
    #[must_use]
    pub fn parse(text: &str) -> Option<Self> {
        if text.is_empty() {
            return None;
        }

        if size::of_text(text) > u64::from(limits::ACCOUNT_ID_BYTES_MAX) {
            return None;
        }

        if !text.bytes().all(byte_is_account) {
            return None;
        }

        let account = Self(text.to_owned());

        assert_eq!(account.0, text, "an account id holds the text it was parsed from");

        assert!(
            account.0.bytes().all(byte_is_account),
            "an account id holds only the bytes it may",
        );

        Some(account)
    }

    #[must_use]
    pub fn text(&self) -> &str {
        assert!(!self.0.is_empty(), "an account id is never empty");

        assert!(
            size::of_text(&self.0) <= u64::from(limits::ACCOUNT_ID_BYTES_MAX),
            "an account id never outgrows its budget",
        );

        self.0.as_str()
    }
}

impl AttachmentID {
    #[must_use]
    pub fn parse(text: &str) -> Option<Self> {
        let digits = text.strip_prefix(ATTACHMENT_PREFIX).unwrap_or(text);

        if !digits.bytes().all(|byte| byte.is_ascii_digit()) {
            return None;
        }

        if digits.parse::<i64>().is_err() {
            return None;
        }

        let id = Self(text.to_owned());

        assert_eq!(id.0, text, "an attachment id holds the text it was parsed from");

        assert!(
            id.0.bytes().all(byte_is_attachment),
            "an attachment id holds only the bytes it may",
        );

        Some(id)
    }

    #[must_use]
    pub fn text(&self) -> &str {
        assert!(!self.0.is_empty(), "an attachment id is never empty");
        assert!(self.0.ends_with(|found: char| found.is_ascii_digit()), "an id ends in its number");

        self.0.as_str()
    }
}

impl BlogPostID {
    #[must_use]
    pub const fn get(self) -> i64 {
        assert!(self.0 >= 1, "a blog post id is positive");

        self.0
    }

    #[must_use]
    pub fn new(raw: i64) -> Option<Self> {
        let id = (raw >= 1).then_some(Self(raw));

        assert_eq!(id.is_some(), raw >= 1, "a blog post id exists only when positive");

        assert!(
            id.is_none_or(|found| found.0 == raw),
            "a blog post id holds the number it was given",
        );

        id
    }
}

impl CommentID {
    #[must_use]
    pub const fn get(self) -> i64 {
        assert!(self.0 >= 1, "a comment id is positive");

        self.0
    }

    #[must_use]
    pub fn new(raw: i64) -> Option<Self> {
        let id = (raw >= 1).then_some(Self(raw));

        assert_eq!(id.is_some(), raw >= 1, "a comment id exists only when positive");

        assert!(
            id.is_none_or(|found| found.0 == raw),
            "a comment id holds the number it was given",
        );

        id
    }
}

impl PageID {
    #[must_use]
    pub const fn get(self) -> i64 {
        assert!(self.0 >= 1, "a page id is positive");

        self.0
    }

    #[must_use]
    pub fn new(raw: i64) -> Option<Self> {
        let id = (raw >= 1).then_some(Self(raw));

        assert_eq!(id.is_some(), raw >= 1, "a page id exists only when positive");
        assert!(id.is_none_or(|found| found.0 == raw), "a page id holds the number it was given");

        id
    }
}

impl SpaceID {
    #[must_use]
    pub const fn get(self) -> i64 {
        assert!(self.0 >= 1, "a space id is positive");

        self.0
    }

    #[must_use]
    pub fn new(raw: i64) -> Option<Self> {
        let id = (raw >= 1).then_some(Self(raw));

        assert_eq!(id.is_some(), raw >= 1, "a space id exists only when positive");
        assert!(id.is_none_or(|found| found.0 == raw), "a space id holds the number it was given");

        id
    }
}

impl SpaceKey {
    #[must_use]
    pub fn is_personal(&self) -> bool {
        assert!(!self.0.is_empty(), "a space key is never empty");
        assert!(self.0.len() >= 2 || !self.0.starts_with('~'), "a personal key names its owner");

        self.0.starts_with(SPACE_KEY_PERSONAL)
    }

    #[must_use]
    pub fn parse(text: &str) -> Option<Self> {
        let body = text.strip_prefix(SPACE_KEY_PERSONAL).unwrap_or(text);

        if body.is_empty() {
            return None;
        }

        if size::of_text(text) > u64::from(limits::SPACE_KEY_BYTES_MAX) {
            return None;
        }

        if !body.bytes().all(byte_is_space_key) {
            return None;
        }

        let key = Self(text.to_owned());

        assert_eq!(key.0, text, "a space key holds the text it was parsed from");
        assert!(body.bytes().all(byte_is_space_key), "a space key holds only the bytes it may");

        Some(key)
    }

    #[must_use]
    pub fn text(&self) -> &str {
        assert!(!self.0.is_empty(), "a space key is never empty");

        assert!(
            size::of_text(&self.0) <= u64::from(limits::SPACE_KEY_BYTES_MAX),
            "a space key never outgrows its budget",
        );

        self.0.as_str()
    }
}

impl Borrow<str> for SpaceKey {
    fn borrow(&self) -> &str {
        self.text()
    }
}

impl fmt::Display for AccountID {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.text())
    }
}

impl fmt::Display for AttachmentID {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.text())
    }
}

impl fmt::Display for BlogPostID {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.get())
    }
}

impl fmt::Display for CommentID {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.get())
    }
}

impl fmt::Display for PageID {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.get())
    }
}

impl fmt::Display for SpaceID {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.get())
    }
}

impl fmt::Display for SpaceKey {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.text())
    }
}

const fn byte_is_account(byte: u8) -> bool {
    let is_account = byte.is_ascii_alphanumeric() || matches!(byte, b':' | b'-' | b'_');

    if is_account {
        assert!(byte != b'"', "an account id can never close a CQL string");
        assert!(byte != b'&', "an account id can never append a query parameter");
    }

    is_account
}

const fn byte_is_attachment(byte: u8) -> bool {
    let is_attachment = byte.is_ascii_digit() || matches!(byte, b'a' | b't');

    if is_attachment {
        assert!(byte != b'/', "an attachment id can never climb out of its directory");
        assert!(byte != b'.', "an attachment id can never name a parent directory");
    }

    is_attachment
}

const fn byte_is_space_key(byte: u8) -> bool {
    let is_space_key = byte.is_ascii_alphanumeric();

    if is_space_key {
        assert!(byte != b'"', "a space key can never close a CQL string");
        assert!(byte != b'/', "a space key can never climb out of its directory");
    }

    is_space_key
}

#[cfg(test)]
mod tests {
    use core::borrow::Borrow as _;

    use super::{
        AccountID,
        AttachmentID,
        BlogPostID,
        CommentID,
        PageID,
        SpaceID,
        SpaceKey,
        byte_is_account,
        byte_is_attachment,
        byte_is_space_key,
    };
    use crate::limits;
    use crate::size;

    #[test]
    fn a_handle_at_its_byte_limit_is_held_and_one_byte_more_is_refused() {
        let account = size::offset(u64::from(limits::ACCOUNT_ID_BYTES_MAX));
        let space = size::offset(u64::from(limits::SPACE_KEY_BYTES_MAX));
        let personal = space.strict_sub(1);

        assert!(AccountID::parse(&"a".repeat(account)).is_some(), "an account at the limit");
        assert!(AccountID::parse(&"a".repeat(account + 1)).is_none(), "an account past it");
        assert!(SpaceKey::parse(&"A".repeat(space)).is_some(), "a space key at the limit");
        assert!(SpaceKey::parse(&"A".repeat(space + 1)).is_none(), "a space key past it");
        assert!(SpaceKey::parse(&format!("~{}", "a".repeat(personal))).is_some(), "personal");
        assert!(SpaceKey::parse(&format!("~{}", "a".repeat(space))).is_none(), "past it");
    }

    #[test]
    fn a_numeric_id_exists_only_when_it_is_positive() {
        assert_eq!(PageID::new(7).map(PageID::get), Some(7), "a positive id survives");
        assert_eq!(PageID::new(0), None, "zero is no id");
        assert_eq!(PageID::new(-3), None, "a negative number is no id");
        assert_eq!(PageID::new(12).map(|id| id.to_string()).as_deref(), Some("12"), "it shows");
        assert_eq!(BlogPostID::new(13).map(|id| id.to_string()).as_deref(), Some("13"), "shows");
        assert_eq!(CommentID::new(14).map(|id| id.to_string()).as_deref(), Some("14"), "shows");
        assert_eq!(SpaceID::new(15).map(|id| id.to_string()).as_deref(), Some("15"), "shows");
    }

    #[test]
    fn a_space_key_borrows_as_the_text_a_map_is_searched_with() {
        let key = SpaceKey::parse("DEV").expect("a key");
        let text: &str = key.borrow();

        assert_eq!(text, "DEV", "a key borrows as its text");
    }

    #[test]
    fn a_space_key_reads_as_a_global_or_a_personal_key() {
        for (text, is_personal) in [
            ("DEV", false),
            ("ds", false),
            ("TEAM42", false),
            ("~5570581a2b3c4d5e6f", true),
            ("~jdoe", true),
        ] {
            let key = SpaceKey::parse(text).expect("a space key Confluence issues");

            assert_eq!(key.text(), text, "{text} parses to itself");
            assert_eq!(key.to_string(), text, "{text} displays as itself");
            assert_eq!(key.is_personal(), is_personal, "{text} names whose space it is");
        }
    }

    #[test]
    fn a_space_key_that_could_append_parameters_or_close_a_term_is_refused() {
        let long = over(limits::SPACE_KEY_BYTES_MAX);

        for text in [
            "",
            "~",
            "~~DEV",
            "D~EV",
            "DEV&limit=1",
            "DEV ",
            "MY_SPACE",
            "DE\" OR x",
            long.as_str(),
        ] {
            assert_eq!(SpaceKey::parse(text), None, "{text} is no space key");
        }
    }

    #[test]
    fn an_account_id_reads_in_both_shapes_atlassian_issues() {
        for text in ["5b10ac8d82e05b22cc7d4ef5", "557058:1a2b3c4d-5e6f", "a_b-c:1"] {
            let account = AccountID::parse(text).expect("an account id Atlassian issues");

            assert_eq!(account.text(), text, "{text} parses to itself");
            assert_eq!(account.to_string(), text, "{text} displays as itself");
        }
    }

    #[test]
    fn an_account_id_that_is_a_name_an_email_or_a_term_is_refused() {
        let long = over(limits::ACCOUNT_ID_BYTES_MAX);

        for text in ["", "user@example.com", "Jane Doe", "x\" OR creator = \"Y", long.as_str()] {
            assert_eq!(AccountID::parse(text), None, "{text} is no account id");
        }
    }

    #[test]
    fn an_attachment_id_reads_with_or_without_its_prefix() {
        for text in ["att123456", "123456", "att1"] {
            let id = AttachmentID::parse(text).expect("an attachment id Confluence issues");

            assert_eq!(id.text(), text, "{text} parses to itself");
            assert_eq!(id.to_string(), text, "{text} displays as itself");
        }
    }

    #[test]
    fn an_attachment_id_that_is_no_number_or_overflows_is_refused() {
        let overflow = format!("att{}0", i64::MAX);

        for text in ["", "att", "ATT1", "att-1", "att+1", "att1/../x", "1.5", overflow.as_str()] {
            assert_eq!(AttachmentID::parse(text), None, "{text} is no attachment id");
        }
    }

    #[test]
    fn every_byte_a_handle_admits_is_one_its_specification_names() {
        for byte in 0..=u8::MAX {
            let is_word = byte.is_ascii_alphanumeric();

            assert_eq!(byte_is_space_key(byte), is_word, "{byte} in a space key");

            assert_eq!(
                byte_is_attachment(byte),
                byte.is_ascii_digit() || b"at".contains(&byte),
                "{byte} in an attachment id",
            );

            assert_eq!(
                byte_is_account(byte),
                is_word || b":-_".contains(&byte),
                "{byte} in an account",
            );

            assert!(!byte_is_account(byte) || byte.is_ascii(), "{byte}: an account id is ASCII");
            assert!(!byte_is_space_key(byte) || byte.is_ascii(), "{byte}: a space key is ASCII");
        }
    }

    #[test]
    fn every_numeric_handle_gives_back_the_number_it_holds() {
        assert_eq!(BlogPostID::new(7).expect("an id").get(), 7, "a blog post");
        assert_eq!(CommentID::new(8).expect("an id").get(), 8, "a comment");
        assert_eq!(PageID::new(9).expect("an id").get(), 9, "a page");
        assert_eq!(SpaceID::new(10).expect("an id").get(), 10, "a space");
    }

    fn over(bytes_max: u32) -> String {
        assert!(bytes_max >= 1, "a byte budget holds at least one byte");

        "A".repeat(size::offset(u64::from(bytes_max).strict_add(1)))
    }
}
