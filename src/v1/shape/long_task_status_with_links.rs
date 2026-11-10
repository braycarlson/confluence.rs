#[derive(Clone, Copy, Debug)]
pub struct LongTaskStatusWithLinks;

impl LongTaskStatusWithLinks {
    pub const ADDITIONAL_DETAILS: crate::Field<
        Self,
        crate::Object<super::LongTaskStatusWithLinksAdditionalDetails>,
    > = crate::Field::new("additionalDetails");

    pub const ARI: crate::Field<Self, crate::Text> = crate::Field::new("ari");
    pub const ELAPSED_TIME: crate::Field<Self, crate::Integer> = crate::Field::new("elapsedTime");

    pub const ERRORS: crate::Field<Self, crate::List<crate::Object<super::Message>>> =
        crate::Field::new("errors");

    pub const FINISHED: crate::Field<Self, crate::Boolean> = crate::Field::new("finished");
    pub const ID: crate::Field<Self, crate::Text> = crate::Field::new("id");
    pub const LINKS: crate::Field<Self, crate::Map<crate::Any>> = crate::Field::new("_links");

    pub const MESSAGES: crate::Field<Self, crate::List<crate::Object<super::Message>>> =
        crate::Field::new("messages");

    pub const NAME: crate::Field<Self, crate::Object<super::LongTaskStatusWithLinksName>> =
        crate::Field::new("name");

    pub const PERCENTAGE_COMPLETE: crate::Field<Self, crate::Integer> =
        crate::Field::new("percentageComplete");

    pub const STATUS: crate::Field<Self, crate::Text> = crate::Field::new("status");
    pub const SUCCESSFUL: crate::Field<Self, crate::Boolean> = crate::Field::new("successful");
}
