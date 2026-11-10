#[derive(Clone, Copy, Debug)]
pub struct AddUserToGroupByGroupID<'call> {
    pub body: &'call serde_json::Value,
    pub group_id: &'call str,
}

impl crate::Operation for AddUserToGroupByGroupID<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Post;
    const PATH: &'static str = "/wiki/rest/api/group/userByGroupId";

    type Options<'options> = ();
    type Reply = crate::Nothing;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("groupId", Some(self.group_id));
        request.body_json(self.body)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
