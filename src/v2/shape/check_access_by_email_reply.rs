#[derive(Clone, Copy, Debug)]
pub struct CheckAccessByEmailReply;

impl CheckAccessByEmailReply {
    pub const INVALID_EMAILS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("invalidEmails");

    pub const NO_ACCESS_EMAILS: crate::Field<Self, crate::List<crate::Text>> =
        crate::Field::new("noAccessEmails");
}
