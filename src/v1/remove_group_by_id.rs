#[derive(Clone, Copy, Debug)]
pub struct RemoveGroupByID<'call> {
    pub id: &'call str,
}

impl crate::Operation for RemoveGroupByID<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = false;
    const METHOD: crate::Method = crate::Method::Delete;
    const PATH: &'static str = "/wiki/rest/api/group/by-id";

    type Options<'options> = ();
    type Reply = crate::Nothing;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_text("id", Some(self.id));

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
