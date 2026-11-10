#[derive(Clone, Copy, Debug)]
pub struct AddGroupToContentRestrictionByGroupID<'call> {
    pub group_id: &'call str,
    pub id: &'call str,
    pub operation_key: &'call str,
}

impl crate::Operation for AddGroupToContentRestrictionByGroupID<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Put;

    const PATH: &'static str =
        "/wiki/rest/api/content/{id}/restriction/byOperation/{operationKey}/byGroupId/{groupId}";

    type Options<'options> = ();
    type Reply = crate::Nothing;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("groupId", self.group_id)?;
        request.path_text("id", self.id)?;
        request.path_text("operationKey", self.operation_key)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
