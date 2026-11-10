use core::fmt;

use crate::encoding;
use crate::error::Error;

#[non_exhaustive]
pub enum Credentials<'credentials> {
    Bearer { cloud_id: &'credentials str, token: &'credentials str },
    Token { cloud_id: &'credentials str, email: &'credentials str, token: &'credentials str },
}

impl Credentials<'_> {
    pub(crate) fn authorization(&self) -> Result<String, Error> {
        let authorization = match self {
            Self::Bearer { cloud_id: _, token } => {
                token_check(token)?;

                format!("Bearer {token}")
            }
            Self::Token { cloud_id: _, email, token } => {
                token_check(token)?;

                if email.is_empty() {
                    return Err(Error::Input("the account email is empty".to_owned()));
                }

                let pair = format!("{email}:{token}");

                format!("Basic {}", encoding::base64(pair.as_bytes()))
            }
        };

        assert!(authorization.contains(' '), "an authorization names its scheme");
        assert!(!authorization.ends_with(' '), "an authorization carries a credential");

        Ok(authorization)
    }

    pub(crate) fn cloud_id(&self) -> Result<&str, Error> {
        let (Self::Bearer { cloud_id, token: _ } | Self::Token { cloud_id, email: _, token: _ }) =
            self;

        if cloud_id.is_empty() {
            return Err(Error::Input("the cloud id is empty".to_owned()));
        }

        if !cloud_id.bytes().all(byte_is_cloud_id) {
            return Err(Error::Input(format!("the cloud id {cloud_id} is not a site id")));
        }

        assert!(!cloud_id.is_empty(), "a checked cloud id names a site");
        assert!(!cloud_id.contains('/'), "a checked cloud id never leaves its path segment");
        assert!(cloud_id.is_ascii(), "a checked cloud id is plain text");

        Ok(cloud_id)
    }
}

impl fmt::Debug for Credentials<'_> {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut debug = formatter.debug_struct("Credentials");

        match self {
            Self::Bearer { cloud_id, token: _ } => {
                debug.field("cloud_id", cloud_id).finish_non_exhaustive()
            }
            Self::Token { cloud_id, email, token: _ } => {
                debug.field("cloud_id", cloud_id).field("email", email).finish_non_exhaustive()
            }
        }
    }
}

const fn byte_is_cloud_id(byte: u8) -> bool {
    if byte.is_ascii_alphanumeric() {
        return true;
    }

    assert!(!byte.is_ascii_alphanumeric(), "a letter or a digit already passed");

    byte == b'-'
}

fn token_check(token: &str) -> Result<(), Error> {
    if token.is_empty() {
        return Err(Error::Input("the token is empty".to_owned()));
    }

    if token.trim() != token {
        return Err(Error::Input("the token has space around it".to_owned()));
    }

    if token.contains(['\r', '\n']) {
        return Err(Error::Input("the token holds a line break".to_owned()));
    }

    assert!(!token.is_empty(), "a checked token carries a credential");
    assert!(!token.contains('\n'), "a checked token stays on its header line");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::Credentials;

    const TOKEN: Credentials<'static> = Credentials::Token {
        cloud_id: "11111111-2222-3333-4444-555555555555",
        email: "user@example.com",
        token: "token123",
    };

    #[test]
    fn a_bearer_token_passes_through_unencoded() {
        let bearer = Credentials::Bearer { cloud_id: "site", token: "access" };

        assert_eq!(bearer.authorization().expect("a token"), "Bearer access", "the scheme");
    }

    #[test]
    fn a_site_that_could_not_be_reached_is_refused() {
        let hostile = Credentials::Bearer { cloud_id: "tenant/../other", token: "x" };
        let blank = Credentials::Bearer { cloud_id: "", token: "x" };

        assert!(hostile.cloud_id().is_err(), "a separator never reaches a URL");
        assert!(blank.cloud_id().is_err(), "an empty site is refused");
    }

    #[test]
    fn a_token_pair_encodes_as_http_basic_expects() {
        assert_eq!(
            TOKEN.authorization().expect("a pair"),
            "Basic dXNlckBleGFtcGxlLmNvbTp0b2tlbjEyMw==",
            "the pair is base64 of email, colon, token",
        );
    }

    #[test]
    fn a_token_that_could_not_be_sent_is_refused() {
        let tokenless = Credentials::Bearer { cloud_id: "site", token: "" };
        let unnamed = Credentials::Token { cloud_id: "site", email: "", token: "x" };
        let split = Credentials::Bearer { cloud_id: "site", token: "a\r\nX-Injected: yes" };
        let padded = Credentials::Bearer { cloud_id: "site", token: "access " };

        assert!(tokenless.authorization().is_err(), "an empty token is refused");
        assert!(unnamed.authorization().is_err(), "an empty email is refused");
        assert!(split.authorization().is_err(), "a token never starts a new header");
        assert!(padded.authorization().is_err(), "a pasted space is refused, not a crash");
    }

    #[test]
    fn the_debug_form_never_prints_the_token() {
        let printed = format!("{TOKEN:?}");

        assert!(printed.contains("user@example.com"), "the account is printed");
        assert!(!printed.contains("token123"), "the secret is not");
    }
}
