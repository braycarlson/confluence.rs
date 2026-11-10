#[derive(Clone, Copy, Debug)]
pub struct GetSpacePermissionTransitionTaskStatus<'call> {
    pub task_id: &'call str,
}

impl crate::Operation for GetSpacePermissionTransitionTaskStatus<'_> {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/api/v2/space-permissions/transition/tasks/{taskId}";

    type Options<'options> = ();
    type Reply = crate::Object<super::shape::BulkTransitionTaskStatusResponse>;

    fn request(
        &self,
        _options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.path_text("taskId", self.task_id)?;

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}
