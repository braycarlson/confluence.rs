use core::error;
use core::fmt;

const STATUS_MAX: u16 = 599;
const STATUS_MIN: u16 = 100;
const _: () = assert!(STATUS_MIN < STATUS_MAX, "the status range is ordered");

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Confluence { message: String, status: u16 },
    Indefinite(String),
    Input(String),
    Reply(String),
    Transport(String),
}

impl Error {
    #[must_use]
    pub fn message(&self) -> &str {
        let message = match self {
            Self::Confluence { message, status: _ }
            | Self::Indefinite(message)
            | Self::Input(message)
            | Self::Reply(message)
            | Self::Transport(message) => message.as_str(),
        };

        assert!(!message.is_empty(), "an error always says what went wrong");

        message
    }

    #[must_use]
    pub fn status(&self) -> Option<u16> {
        let Self::Confluence { message: _, status } = self else {
            return None;
        };

        assert!(*status >= STATUS_MIN, "a refusal carries a real status");
        assert!(*status <= STATUS_MAX, "a refusal carries a real status");

        Some(*status)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.message())
    }
}

impl error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::Error;

    #[test]
    #[should_panic(expected = "an error always says what went wrong")]
    fn an_error_with_no_message_is_a_programmer_error() {
        let silent = Error::Reply(String::new());
        let message = silent.message();

        assert!(!message.is_empty(), "the panic above is the point of this test");
    }

    #[test]
    fn each_kind_displays_its_message_and_nothing_else() {
        let errors = [
            Error::Confluence { message: "c".to_owned(), status: 404 },
            Error::Indefinite("a".to_owned()),
            Error::Input("b".to_owned()),
            Error::Reply("d".to_owned()),
            Error::Transport("e".to_owned()),
        ];

        for (error, expected) in errors.iter().zip(["c", "a", "b", "d", "e"]) {
            assert_eq!(error.message(), expected, "the message is the one it was built with");
            assert_eq!(error.to_string(), expected, "the display is the message alone");
        }
    }

    #[test]
    fn only_a_refusal_carries_a_status() {
        let refused = Error::Confluence { message: "gone".to_owned(), status: 404 };

        assert_eq!(refused.status(), Some(404), "a refusal names its status");
        assert_eq!(refused.message(), "gone", "a refusal keeps its message");
        assert_eq!(Error::Transport("reset".to_owned()).status(), None, "the network has none");
        assert_eq!(Error::Input("bad".to_owned()).status(), None, "a caller's fault has none");
    }
}
