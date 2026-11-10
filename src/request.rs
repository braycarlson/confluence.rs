use core::fmt;

use serde_json::Value;

use crate::encoding;
use crate::error::Error;
use crate::limits;
use crate::method::Method;
use crate::multipart;
use crate::operation::Operation;
use crate::part::Part;
use crate::size;

const ACCEPT_ANY: &str = "*/*";
const ACCEPT_JSON: &str = "application/json";
const CONTENT_JSON: &str = "application/json";
const HEADER_COUNT_MAX: u64 = 16;
const HEADER_XSRF_NAME: &str = "X-Atlassian-Token";
const HEADER_XSRF_VALUE: &str = "no-check";
const _: () = assert!(HEADER_COUNT_MAX >= 2, "a request may carry the headers Confluence names");

pub struct Request {
    accept: &'static str,
    authorization: String,
    body: Vec<u8>,
    content_type: Option<String>,
    headers: Vec<(&'static str, String)>,
    is_read: bool,
    method: Method,
    url: String,
}

impl Request {
    #[must_use]
    pub fn accept(&self) -> &'static str {
        assert!(!self.accept.is_empty(), "a request names what it accepts");
        assert!(self.accept.contains('/'), "an accepted type is a media type");

        self.accept
    }

    pub(crate) fn accept_any(&mut self) {
        assert_eq!(self.accept, ACCEPT_JSON, "a request asks for one reply type");

        self.accept = ACCEPT_ANY;

        assert_eq!(self.accept, ACCEPT_ANY, "a download takes whatever Confluence sends");
        assert_ne!(self.accept, ACCEPT_JSON, "a download asks for nothing narrower");
    }

    #[must_use]
    pub fn authorization(&self) -> &str {
        assert!(!self.authorization.is_empty(), "a request carries its credential");
        assert!(self.authorization.contains(' '), "an authorization names its scheme");

        self.authorization.as_str()
    }

    #[must_use]
    pub fn body(&self) -> &[u8] {
        assert!(size::of_slice(&self.body) <= limits::UPLOAD_BYTES_MAX, "a body fits its limit");

        if !self.body.is_empty() {
            assert!(self.content_type.is_some(), "a body names its type");
        }

        self.body.as_slice()
    }

    pub fn body_json(&mut self, body: &Value) -> Result<(), Error> {
        assert!(self.content_type.is_none(), "a request carries one body");

        let bytes = serde_json::to_vec(body).map_err(|cause| {
            Error::Input(format!("the request body cannot be written: {cause}"))
        })?;

        self.body_set(bytes, CONTENT_JSON.to_owned())
    }

    pub fn body_multipart(&mut self, parts: &[Part<'_>]) -> Result<(), Error> {
        assert!(self.content_type.is_none(), "a request carries one body");

        if parts.is_empty() {
            return Err(Error::Input("a multipart body holds no parts".to_owned()));
        }

        if parts.iter().any(|part| part.name.is_empty()) {
            return Err(Error::Input("a multipart part has no name".to_owned()));
        }

        let boundary = multipart::boundary(parts)?;
        let bytes = multipart::encode(parts, &boundary);

        self.body_set(bytes, format!("multipart/form-data; boundary={boundary}"))?;
        self.header(HEADER_XSRF_NAME, HEADER_XSRF_VALUE);

        Ok(())
    }

    fn body_set(&mut self, bytes: Vec<u8>, content_type: String) -> Result<(), Error> {
        assert!(!content_type.is_empty(), "a body names its type");
        assert!(self.content_type.is_none(), "a request carries one body");

        if !body_fits(size::of_slice(&bytes)) {
            return Err(Error::Input(format!(
                "a request body ran past the {} byte limit",
                limits::UPLOAD_BYTES_MAX,
            )));
        }

        self.body = bytes;
        self.content_type = Some(content_type);

        assert!(self.content_type.is_some(), "a body names its type");

        Ok(())
    }

    #[must_use]
    pub fn content_type(&self) -> Option<&str> {
        let found = self.content_type.as_deref();

        assert!(found.is_none_or(|text| !text.is_empty()), "a named type is never empty");

        found
    }

    pub(crate) fn content_type_settle(&mut self) {
        assert!(self.url.starts_with("https://"), "every request is encrypted");

        if self.method == Method::Get {
            assert!(self.content_type.is_none(), "a GET carries no body");
            return;
        }

        let named = self.content_type.get_or_insert_with(|| CONTENT_JSON.to_owned());

        assert!(!named.is_empty(), "a write names a type");
    }

    pub fn header(&mut self, name: &'static str, value: &str) {
        assert!(!name.is_empty(), "a header is named");
        assert!(size::of_slice(&self.headers) < HEADER_COUNT_MAX, "a request stays under its cap");

        self.headers.push((name, value.to_owned()));

        assert!(!self.headers.is_empty(), "a pushed header is held");
    }

    pub fn header_text(&mut self, name: &'static str, value: Option<&str>) -> Result<(), Error> {
        assert!(!name.is_empty(), "a header is named");
        assert!(name.is_ascii(), "a header name is plain");

        let Some(found) = value else {
            return Ok(());
        };

        if found.chars().any(char::is_control) {
            return Err(Error::Input(format!("the header {name} holds a control character")));
        }

        self.header(name, found);

        Ok(())
    }

    #[must_use]
    pub fn headers(&self) -> &[(&'static str, String)] {
        assert!(size::of_slice(&self.headers) <= HEADER_COUNT_MAX, "a request stays under its cap");

        self.headers.as_slice()
    }

    #[must_use]
    pub(crate) fn is_blank(&self) -> bool {
        assert!(!self.url.is_empty(), "a request names where it goes");
        assert!(self.headers.is_empty(), "a blank request carries no header yet");
        assert!(self.url.starts_with("https://"), "every request is encrypted");

        if self.url.contains('?') {
            return false;
        }

        self.body.is_empty()
    }

    #[must_use]
    pub(crate) fn is_complete(&self) -> bool {
        assert!(!self.url.is_empty(), "a request names where it goes");
        assert!(self.url.starts_with("https://"), "every request is encrypted");

        !self.url.contains('{')
    }

    #[must_use]
    pub fn is_read(&self) -> bool {
        if self.method == Method::Get {
            assert!(self.is_read, "a GET is always a read");
        }

        self.is_read
    }

    #[must_use]
    pub fn method(&self) -> Method {
        if self.method == Method::Get {
            assert!(self.is_read, "a GET is always a read");
        }

        self.method
    }

    #[must_use]
    pub(crate) fn of<Called>(url: String, authorization: &str) -> Self
    where
        Called: Operation,
    {
        assert!(url.starts_with("https://"), "every request is encrypted");
        assert!(!authorization.is_empty(), "a request carries its credential");

        Self {
            accept: ACCEPT_JSON,
            authorization: authorization.to_owned(),
            body: Vec::new(),
            content_type: None,
            headers: Vec::new(),
            is_read: Called::IS_READ,
            method: Called::METHOD,
            url,
        }
    }

    pub fn path_number(&mut self, name: &'static str, value: i64) -> Result<(), Error> {
        assert!(!name.is_empty(), "a path parameter is named");
        assert!(self.url.contains(&format!("{{{name}}}")), "the path names {name}");

        self.path_text(name, &value.to_string())
    }

    pub fn path_text(&mut self, name: &'static str, value: &str) -> Result<(), Error> {
        assert!(!name.is_empty(), "a path parameter is named");

        if value.is_empty() {
            return Err(Error::Input(format!("the path parameter {name} is empty")));
        }

        if matches!(value, "." | "..") {
            return Err(Error::Input(format!("the path parameter {name} climbs the path")));
        }

        let placeholder = format!("{{{name}}}");

        assert!(self.url.contains(&placeholder), "the path names {name}");

        self.url = self.url.replace(&placeholder, &encoding::percent(value));

        assert!(!self.url.contains(&placeholder), "a filled parameter leaves no placeholder");
        assert!(!self.url.is_empty(), "a filled path still names where it goes");

        Ok(())
    }

    pub fn query_boolean(&mut self, name: &'static str, value: Option<bool>) {
        assert!(!name.is_empty(), "a query parameter is named");
        assert!(self.url.starts_with("https://"), "every request is encrypted");

        if let Some(found) = value {
            self.query_term(name, if found { "true" } else { "false" });
        }
    }

    pub fn query_number(&mut self, name: &'static str, value: Option<i64>) {
        assert!(!name.is_empty(), "a query parameter is named");
        assert!(self.url.starts_with("https://"), "every request is encrypted");

        if let Some(found) = value {
            self.query_term(name, &found.to_string());
        }
    }

    pub fn query_numbers(&mut self, name: &'static str, values: Option<&[i64]>) {
        assert!(!name.is_empty(), "a query parameter is named");
        assert!(self.url.starts_with("https://"), "every request is encrypted");

        for found in values.unwrap_or_default() {
            self.query_term(name, &found.to_string());
        }
    }

    #[must_use]
    pub fn query_of(&self, name: &str) -> Option<String> {
        assert!(!name.is_empty(), "a query parameter is named");
        assert!(self.url.starts_with("https://"), "every request is encrypted");

        let (_path, query) = self.url.split_once('?')?;
        let prefix = format!("{name}=");
        let term = query.split('&').find_map(|found| found.strip_prefix(prefix.as_str()))?;

        String::from_utf8(encoding::percent_decode(term)).ok()
    }

    pub fn query_pairs(
        &mut self,
        name: &'static str,
        pairs: Option<&[(&str, &str)]>,
    ) -> Result<(), Error> {
        assert!(!name.is_empty(), "a query parameter is named");
        assert!(self.url.starts_with("https://"), "every request is encrypted");

        let held = pairs.unwrap_or_default();

        if let Some((key, _value)) = held.iter().find(|(key, _value)| !key_is_sendable(key)) {
            return Err(Error::Input(format!(
                "the object {name} holds a key it cannot send: {key:?}"
            )));
        }

        for (key, value) in held {
            self.query_push(name, Some(key), &escaped(value));
        }

        Ok(())
    }

    fn query_push(&mut self, name: &'static str, key: Option<&str>, escaped: &str) {
        assert!(!name.is_empty(), "a query parameter is named");
        assert!(key.is_none_or(key_is_sendable), "a key arrives framable");
        assert!(!escaped.contains(['&', '#', ' ']), "a value arrives escaped");

        let framed = key.map_or_else(|| name.to_owned(), |found| format!("{name}[{found}]"));

        self.url.push(if self.url.contains('?') { '&' } else { '?' });
        self.url.push_str(&encoding::percent(&framed));
        self.url.push('=');
        self.url.push_str(escaped);

        assert!(self.url.contains('?'), "a term follows the query mark");
        assert!(!self.url.ends_with('&'), "a term follows its separator");
    }

    pub(crate) fn query_replace(&mut self, name: &'static str, value: &str) {
        assert!(!name.is_empty(), "a query parameter is named");
        assert!(!value.is_empty(), "a cursor is never empty");

        let prefix = format!("{name}=");

        if let Some((path, query)) = self.url.split_once('?') {
            let terms: Vec<&str> =
                query.split('&').filter(|term| !term.starts_with(&prefix)).collect();

            self.url = if terms.is_empty() {
                path.to_owned()
            } else {
                format!("{path}?{}", terms.join("&"))
            };
        }

        self.query_term(name, value);

        let query = self.url.split_once('?').map_or("", |(_path, found)| found);
        let held = query.split('&').filter(|term| term.starts_with(&prefix)).count();

        assert_eq!(held, 1, "a replaced term appears once");
    }

    fn query_term(&mut self, name: &'static str, value: &str) {
        assert!(!name.is_empty(), "a query parameter is named");
        assert!(self.url.starts_with("https://"), "every request is encrypted");

        self.query_push(name, None, &escaped(value));
    }

    pub fn query_text(&mut self, name: &'static str, value: Option<&str>) {
        assert!(!name.is_empty(), "a query parameter is named");
        assert!(self.url.starts_with("https://"), "every request is encrypted");

        if let Some(found) = value {
            self.query_term(name, found);
        }
    }

    pub fn query_texts(&mut self, name: &'static str, values: Option<&[&str]>) {
        assert!(!name.is_empty(), "a query parameter is named");
        assert!(self.url.starts_with("https://"), "every request is encrypted");

        for found in values.unwrap_or_default() {
            self.query_term(name, found);
        }
    }

    pub fn query_texts_joined(
        &mut self,
        name: &'static str,
        values: Option<&[&str]>,
    ) -> Result<(), Error> {
        assert!(!name.is_empty(), "a query parameter is named");
        assert!(self.url.starts_with("https://"), "every request is encrypted");

        let listed = values.unwrap_or_default();

        if listed.is_empty() {
            return Ok(());
        }

        if let Some(found) = listed.iter().find(|value| value.contains(',')) {
            return Err(Error::Input(format!("the list {name} holds a comma inside {found:?}")));
        }

        let joined: Vec<String> = listed.iter().map(|value| escaped(value)).collect();

        self.query_push(name, None, &joined.join(","));

        Ok(())
    }

    #[must_use]
    pub fn url(&self) -> &str {
        assert!(self.url.starts_with("https://"), "every request is encrypted");
        assert!(!self.url.contains(' '), "a URL holds no bare space");

        self.url.as_str()
    }

    pub(crate) fn url_check(&self) -> Result<(), Error> {
        assert!(self.is_complete(), "every path parameter was filled");

        if size::of_text(&self.url) > u64::from(limits::URL_BYTES_MAX) {
            return Err(Error::Input(format!(
                "a request URL ran past the {} byte limit",
                limits::URL_BYTES_MAX,
            )));
        }

        assert!(self.url.starts_with("https://"), "every request is encrypted");

        Ok(())
    }
}

impl fmt::Debug for Request {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("Request")
            .field("method", &self.method)
            .field("url", &self.url)
            .finish_non_exhaustive()
    }
}

#[must_use]
fn body_fits(byte_count: u64) -> bool {
    assert!(usize::try_from(byte_count).is_ok(), "a body of {byte_count} bytes fits in memory");

    byte_count <= limits::UPLOAD_BYTES_MAX
}

#[must_use]
fn escaped(value: &str) -> String {
    if value.is_empty() {
        return String::new();
    }

    let text = encoding::percent(value);

    assert!(text.is_ascii(), "an escaped value is plain");

    text
}

#[must_use]
fn key_is_sendable(key: &str) -> bool {
    !key.is_empty() && !key.contains(['[', ']'])
}

#[cfg(test)]
pub(crate) mod tests {
    use serde_json::json;

    use super::{Request, body_fits};
    use crate::base::Base;
    use crate::encoding;
    use crate::error::Error;
    use crate::limits;
    use crate::method::Method;
    use crate::operation::Operation;
    use crate::part::Part;
    use crate::size;

    const AUTHORIZATION: &str = "Basic secret";
    const URL: &str = "https://api.atlassian.com/ex/confluence/site/wiki/api/v2/pages/{id}";

    #[derive(Debug)]
    struct Fixture;

    #[derive(Debug)]
    struct FixtureWrite;

    impl Operation for Fixture {
        const BASE: Base = Base::Confluence;
        const IS_READ: bool = true;
        const METHOD: Method = Method::Get;
        const PATH: &'static str = "/wiki/api/v2/pages/{id}";

        type Options<'options> = ();
        type Reply = ();

        fn request(&self, _options: &(), request: &mut Request) -> Result<(), Error> {
            assert!(request.is_blank(), "a request starts blank");
            assert!(!request.url().is_empty(), "a request names where it goes");

            Ok(())
        }
    }

    impl Operation for FixtureWrite {
        const BASE: Base = Base::Confluence;
        const IS_READ: bool = false;
        const METHOD: Method = Method::Post;
        const PATH: &'static str = "/wiki/api/v2/pages";

        type Options<'options> = ();
        type Reply = ();

        fn request(&self, _options: &(), request: &mut Request) -> Result<(), Error> {
            assert!(request.is_blank(), "a request starts blank");
            assert!(!request.url().is_empty(), "a request names where it goes");

            Ok(())
        }
    }

    #[test]
    fn a_bodiless_write_says_json_and_nothing_else_changes_its_type() {
        let mut read = blank_request();
        let mut upload = write_request();
        let mut write = write_request();

        let part =
            Part { bytes: b"\x89PNG", content_type: "image/png", file_name: None, name: "file" };

        upload.body_multipart(&[part]).expect("fits");

        let boundary = upload.content_type().expect("a type").to_owned();

        read.content_type_settle();
        upload.content_type_settle();
        write.content_type_settle();

        assert_eq!(read.content_type(), None, "a GET names no type");
        assert_eq!(upload.content_type(), Some(boundary.as_str()), "a body keeps its own type");
        assert_eq!(write.content_type(), Some("application/json"), "a write says JSON");
        assert!(write.body().is_empty(), "the type adds no body");
    }

    #[test]
    fn a_body_at_its_limit_fits_and_one_byte_more_does_not() {
        assert!(body_fits(0), "an empty body fits");
        assert!(body_fits(limits::UPLOAD_BYTES_MAX), "a body at the limit fits");
        assert!(!body_fits(limits::UPLOAD_BYTES_MAX.strict_add(1)), "one byte more does not");
    }

    #[test]
    fn a_comma_list_is_one_term_and_a_value_holding_a_comma_is_refused() {
        let mut request = blank_request();

        request.query_texts_joined("expand", Some(&["body.storage", "a b"])).expect("a list");
        request.query_texts_joined("status", Some(&[])).expect("an empty list");

        let refused = blank_request().query_texts_joined("expand", Some(&["a,b"]));
        let read = request.query_of("expand");

        assert!(request.url().ends_with("{id}?expand=body.storage,a%20b"), "literal commas");
        assert_eq!(read.as_deref(), Some("body.storage,a b"), "the list reads back joined");
        assert_eq!(request.query_of("status"), None, "an empty list sends nothing");
        assert!(matches!(refused, Err(Error::Input(_))), "a comma inside a value is ambiguous");
    }

    #[test]
    fn a_cursor_beside_a_longer_name_that_ends_alike_is_left_alone() {
        let mut request = blank_request();

        request.query_text("precursor", Some("x"));
        request.query_text("cursor", Some("a"));
        request.query_replace("cursor", "b");

        assert!(request.url().ends_with("?precursor=x&cursor=b"), "only the cursor moved");
    }

    #[test]
    fn a_cursor_may_be_the_only_term() {
        let mut request = blank_request();

        request.query_number("start", Some(5));
        request.query_replace("start", "100");

        assert!(request.url().ends_with("{id}?start=100"), "the old term is gone");
    }

    #[test]
    fn a_cursor_replaces_the_term_it_overrides_and_keeps_the_rest() {
        let mut request = blank_request();

        request.query_text("cql", Some("a"));
        request.query_number("start", Some(5));
        request.query_number("limit", Some(50));
        request.query_replace("start", "100");

        assert!(request.url().ends_with("?cql=a&limit=50&start=100"), "one cursor term");
    }

    #[test]
    fn a_deep_object_sends_a_term_per_key_and_refuses_a_key_it_cannot_frame() {
        let mut request = blank_request();
        let pairs = [("status", "done"), ("owner", "a b")];

        request.query_pairs("filter", Some(&pairs)).expect("two pairs");

        let (_path, query) = request.url().split_once('?').expect("a query");
        let decoded = String::from_utf8(encoding::percent_decode(query)).expect("text");

        assert_eq!(decoded, "filter[status]=done&filter[owner]=a b", "each key is bracketed");

        for key in ["", "a]b", "[a"] {
            let refused = blank_request().query_pairs("filter", Some(&[(key, "x")]));

            assert!(matches!(refused, Err(Error::Input(_))), "the key {key:?} is refused");
        }
    }

    #[test]
    fn a_header_is_sent_only_when_set_and_never_with_a_control_character() {
        let mut request = blank_request();

        request.header_text("If-None-Match", None).expect("nothing to send");

        assert!(request.headers().is_empty(), "an unset header is not sent");

        request.header_text("If-None-Match", Some("\"tag\"")).expect("an entity tag");

        let refused = blank_request().header_text("If-None-Match", Some("a\r\nX-Injected: yes"));

        assert_eq!(request.headers(), [("If-None-Match", "\"tag\"".to_owned())], "the header");
        assert!(matches!(refused, Err(Error::Input(_))), "a line break could start a header");
    }

    #[test]
    fn a_json_body_declares_its_type_and_keeps_its_bytes() {
        let mut request = blank_request();

        request.body_json(&json!({"a": 1_i64})).expect("JSON writes");

        assert_eq!(request.body(), b"{\"a\":1}", "the JSON bytes");
        assert_eq!(request.content_type(), Some("application/json"), "JSON");
    }

    #[test]
    fn a_multipart_body_declares_its_boundary_and_disarms_the_xsrf_check() {
        let mut request = blank_request();

        let part = Part {
            bytes: b"hello",
            content_type: "text/plain",
            file_name: Some("a.txt"),
            name: "file",
        };

        request.body_multipart(&[part]).expect("a part");

        let content_type = request.content_type().expect("a type");
        let boundary = "multipart/form-data; boundary=confluence-";

        assert!(content_type.starts_with(boundary), "a boundary");
        assert_eq!(request.headers(), [("X-Atlassian-Token", "no-check".to_owned())], "the header");
    }

    #[test]
    fn a_multipart_body_with_no_parts_or_a_nameless_part_is_refused() {
        let unnamed = Part { bytes: b"x", content_type: "text/plain", file_name: None, name: "" };

        assert!(matches!(blank_request().body_multipart(&[]), Err(Error::Input(_))), "no parts");

        assert!(
            matches!(blank_request().body_multipart(&[unnamed]), Err(Error::Input(_))),
            "a part with no name",
        );
    }

    #[test]
    fn a_path_value_is_escaped_so_it_cannot_leave_its_segment() {
        let mut request = blank_request();

        request.path_text("id", "a/b?c=d").expect("a value");

        assert!(request.url().ends_with("/pages/a%2Fb%3Fc%3Dd"), "every separator is escaped");
        assert!(request.is_complete(), "no placeholder remains");
    }

    #[test]
    fn a_path_value_that_climbs_or_vanishes_is_refused() {
        for value in ["", ".", ".."] {
            let refused = blank_request().path_text("id", value);

            assert!(matches!(refused, Err(Error::Input(_))), "{value:?} is refused");
        }
    }

    #[test]
    fn a_query_term_appears_only_when_it_is_set() {
        let mut request = blank_request();

        request.query_boolean("a", None);
        request.query_number("b", None);
        request.query_numbers("c", None);
        request.query_text("d", None);
        request.query_texts("e", None);
        request.query_texts_joined("f", None).expect("nothing to send");
        request.query_pairs("g", None).expect("nothing to send");

        assert!(request.is_blank(), "nothing was set");

        request.query_boolean("a", Some(true));
        request.query_number("b", Some(-1));
        request.query_numbers("c", Some(&[1, 2]));
        request.query_text("d", Some("x y"));
        request.query_texts("e", Some(&["p", "q"]));
        request.query_texts_joined("f", Some(&["r", "s"])).expect("a list");
        request.query_pairs("g", Some(&[("k", "v")])).expect("a pair");

        assert!(
            request.url().ends_with("?a=true&b=-1&c=1&c=2&d=x%20y&e=p&e=q&f=r,s&g%5Bk%5D=v"),
            "each term is escaped, a list repeats its name, and a joined list is one term",
        );
    }

    #[test]
    fn a_query_term_is_named_exactly_and_a_missing_one_reads_as_absent() {
        let mut request = blank_request();

        assert_eq!(request.query_of("start"), None, "a request with no query holds no term");

        request.query_text("restart", Some("x"));

        assert_eq!(request.query_of("start"), None, "a longer name that ends alike is no match");
        assert_eq!(request.query_of("restart").as_deref(), Some("x"), "the whole name matches");
    }

    #[test]
    fn a_query_term_reads_back_as_it_was_written_whatever_it_holds() {
        for value in ["plain", "a b&c=d", "*all", "space = \"DEV\"", "caf\u{e9}+%2F", "x,y"] {
            let mut request = blank_request();

            request.query_text("before", Some("1"));
            request.query_text("cql", Some(value));
            request.query_text("after", Some("2"));

            assert_eq!(request.query_of("cql").as_deref(), Some(value), "the term {value}");
        }
    }

    #[test]
    fn a_term_or_a_body_ends_a_blank_request() {
        let mut termed = blank_request();
        let mut bodied = blank_request();

        assert!(termed.is_blank(), "a fresh request holds nothing");

        termed.query_text("a", Some("b"));
        bodied.body_json(&json!({"a": "b"})).expect("a body");

        assert!(!termed.is_blank(), "a term has landed");
        assert!(!bodied.is_blank(), "a body has landed");
    }

    #[test]
    fn a_url_at_its_limit_passes_and_one_byte_more_is_refused() {
        let base = "https://api.atlassian.com/";
        let room = size::offset(u64::from(limits::URL_BYTES_MAX)).strict_sub(base.len());
        let fits = format!("{base}{}", "x".repeat(room));
        let long = format!("{fits}x");

        assert!(Request::of::<Fixture>(fits, AUTHORIZATION).url_check().is_ok(), "at the limit");
        assert!(Request::of::<Fixture>(long, AUTHORIZATION).url_check().is_err(), "past it");
    }

    #[test]
    fn an_empty_text_is_a_term_rather_than_a_missing_one() {
        let mut request = blank_request();

        request.query_text("cql", Some(""));
        request.query_texts_joined("expand", Some(&[""])).expect("an empty value");

        assert!(request.url().ends_with("?cql=&expand="), "an empty value is still sent");
    }

    fn blank_request() -> Request {
        let request = Request::of::<Fixture>(URL.to_owned(), AUTHORIZATION);

        assert!(request.is_blank(), "a fixture starts blank");
        assert!(!request.is_complete(), "a fixture holds a placeholder to fill");

        request
    }

    #[test]
    #[should_panic(expected = "the path names unknown")]
    fn filling_a_parameter_the_path_never_named_is_a_programmer_error() {
        let filled = blank_request().path_text("unknown", "x");

        assert!(filled.is_err(), "the panic above is the point of this test");
    }

    pub(crate) fn read_request() -> Request {
        let url = "https://api.atlassian.com/ex/confluence/site/wiki/rest/api/user/current";
        let request = Request::of::<Fixture>(url.to_owned(), AUTHORIZATION);

        assert!(request.is_read(), "a fixture read is a read");
        assert!(request.is_complete(), "a fixture read has nothing to fill");

        request
    }

    #[test]
    fn the_debug_form_never_prints_the_credential() {
        let printed = format!("{:?}", blank_request());

        assert!(printed.contains("/wiki/api/v2/pages/"), "the URL is printed");
        assert!(!printed.contains("secret"), "the credential is not");
    }

    pub(crate) fn write_request() -> Request {
        let url = "https://api.atlassian.com/ex/confluence/site/wiki/api/v2/pages".to_owned();
        let request = Request::of::<FixtureWrite>(url, AUTHORIZATION);

        assert!(!request.is_read(), "a fixture write is a write");
        assert!(request.is_complete(), "a fixture write has nothing to fill");

        request
    }
}
