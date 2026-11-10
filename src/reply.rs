use core::marker::PhantomData;

use serde_json::Value;

use crate::error::Error;
use crate::kind::Kind;
use crate::request::Request;
use crate::response::Response;

#[derive(Clone, Debug)]
pub struct Reply<Body> {
    body: PhantomData<Body>,
    location: Option<String>,
    value: Value,
}

impl<Body> Reply<Body>
where
    Body: Kind,
{
    #[must_use]
    pub fn into_value(self) -> Value {
        if Body::IS_BODY_IGNORED {
            assert!(self.value.is_null(), "a reply of nothing holds nothing");
        }

        self.value
    }

    #[must_use]
    pub fn location(&self) -> Option<&str> {
        let location = self.location.as_deref();

        assert!(location.is_none_or(|found| !found.is_empty()), "a location names somewhere");

        location
    }

    pub(crate) fn parse(response: Response, request: &Request) -> Result<Self, Error> {
        assert!(request.url().starts_with("https://"), "every reply came over an encrypted link");
        assert!(response.status >= 200, "only a success reads as a reply");

        let location = response.location;

        if Body::IS_BODY_IGNORED {
            return Ok(Self { body: PhantomData, location, value: Value::Null });
        }

        if response.body.is_empty() {
            return Ok(Self { body: PhantomData, location, value: Value::Null });
        }

        let parsed = serde_json::from_slice(&response.body);

        if let Ok(value) = parsed {
            return Ok(Self { body: PhantomData, location, value });
        }

        if request.is_read() {
            let url = request.url();
            let cause = parsed.err().map(|found| found.to_string()).unwrap_or_default();

            return Err(Error::Reply(format!("invalid JSON from {url}: {cause}")));
        }

        Ok(Self { body: PhantomData, location, value: Value::Null })
    }

    #[must_use]
    pub fn read(&self) -> Option<Body::Output<'_>> {
        let read = Body::read(&self.value);

        if Body::IS_BODY_IGNORED {
            assert!(self.value.is_null(), "a reply of nothing holds nothing");
        }

        read
    }

    #[must_use]
    pub fn value(&self) -> &Value {
        if Body::IS_BODY_IGNORED {
            assert!(self.value.is_null(), "a reply of nothing holds nothing");
        }

        &self.value
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{Value, json};

    use super::Reply;
    use crate::error::Error;
    use crate::kind::{Any, Nothing, Text};
    use crate::request::tests::{read_request, write_request};
    use crate::response::Response;

    const TASK: &str = "https://site/wiki/rest/api/longtask/10";

    #[test]
    fn a_body_that_is_not_json_is_a_fault_of_the_reply_on_a_read() {
        let parsed = Reply::<Any>::parse(answer(b"<html>"), &read_request());

        assert!(matches!(parsed, Err(Error::Reply(_))), "the reply could not be read");

        let message = parsed.expect_err("a refused body").to_string();

        assert!(message.contains("/wiki/rest/api/"), "the failure names the request");
    }

    #[test]
    fn a_landed_write_stays_a_success_whatever_its_body_holds() {
        let mut accepted = answer(b"<html>a redirect page</html>");

        accepted.location = Some(TASK.to_owned());

        let reply = Reply::<Any>::parse(accepted, &write_request()).expect("the write landed");

        assert_eq!(reply.value(), &Value::Null, "the body could not be read");
        assert_eq!(reply.location(), Some(TASK), "the task survives");
    }

    #[test]
    fn a_reply_keeps_the_location_of_an_accepted_task() {
        let mut response = answer(b"");

        response.location = Some(TASK.to_owned());

        let reply = Reply::<Any>::parse(response, &write_request()).expect("an accepted task");

        assert_eq!(reply.location(), Some(TASK), "the task");
        assert_eq!(reply.value(), &Value::Null, "with no body to read");
        assert_eq!(reply.read(), Some(&Value::Null), "which reads as a null");
    }

    #[test]
    fn a_reply_of_nothing_never_reads_its_body() {
        let reply = Reply::<Nothing>::parse(answer(b"<html>"), &read_request()).expect("ignored");

        assert_eq!(reply.read(), Some(()), "the body was not read");
        assert_eq!(reply.value(), &Value::Null, "nothing was kept");
        assert_eq!(reply.into_value(), Value::Null, "nothing was kept");
    }

    #[test]
    fn a_reply_reads_as_its_kind_and_keeps_its_value() {
        let reply = Reply::<Text>::parse(answer(b"\"words\""), &read_request()).expect("a text");

        assert_eq!(reply.read(), Some("words"), "typed");
        assert_eq!(reply.value(), &json!("words"), "raw");
        assert_eq!(reply.into_value(), json!("words"), "owned");
    }

    #[test]
    fn an_empty_body_reads_as_null_whatever_the_kind() {
        let reply = Reply::<Any>::parse(answer(b""), &read_request()).expect("an empty body");
        let text = Reply::<Text>::parse(answer(b""), &read_request()).expect("an empty body");

        assert_eq!(reply.value(), &Value::Null, "there was nothing to read");
        assert_eq!(reply.read(), Some(&Value::Null), "anything reads a null");
        assert_eq!(text.read(), None, "a null is no text");
    }

    fn answer(body: &[u8]) -> Response {
        assert!(body.len() <= 64, "a fixture answer is short");

        let response = Response {
            body: body.to_vec(),
            is_near_limit: false,
            location: None,
            rate_limit_reason: None,
            retry_after_seconds: None,
            status: 200,
            streamed_count: 0,
        };

        assert_eq!(response.status, 200, "a fixture answer succeeds");

        response
    }
}
