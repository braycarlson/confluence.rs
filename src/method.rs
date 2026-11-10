#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Method {
    Delete,
    Get,
    Patch,
    Post,
    Put,
}

impl Method {
    #[must_use]
    pub const fn text(self) -> &'static str {
        let text = match self {
            Self::Delete => "DELETE",
            Self::Get => "GET",
            Self::Patch => "PATCH",
            Self::Post => "POST",
            Self::Put => "PUT",
        };

        assert!(!text.is_empty(), "a method spells its verb");
        assert!(text.len() <= 6, "the longest verb Confluence uses is six letters");

        text
    }
}

#[cfg(test)]
mod tests {
    use super::Method;

    #[test]
    fn each_method_spells_its_http_verb() {
        let spelled = [
            (Method::Delete, "DELETE"),
            (Method::Get, "GET"),
            (Method::Patch, "PATCH"),
            (Method::Post, "POST"),
            (Method::Put, "PUT"),
        ];

        for (method, text) in spelled {
            assert_eq!(method.text(), text, "the verb is spelled as HTTP spells it");
        }

        assert_ne!(Method::Get, Method::Post, "a read differs from a write");
    }
}
