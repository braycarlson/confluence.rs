#[derive(Clone, Copy, Debug)]
pub struct GetTasks;

#[derive(Clone, Copy, Debug, Default)]
pub struct Options<'options> {
    pub assigned_to: Option<&'options [&'options str]>,
    pub blogpost_id: Option<&'options [i64]>,
    pub body_format: Option<&'options str>,
    pub completed_at_from: Option<i64>,
    pub completed_at_to: Option<i64>,
    pub completed_by: Option<&'options [&'options str]>,
    pub created_at_from: Option<i64>,
    pub created_at_to: Option<i64>,
    pub created_by: Option<&'options [&'options str]>,
    pub cursor: Option<&'options str>,
    pub due_at_from: Option<i64>,
    pub due_at_to: Option<i64>,
    pub include_blank_tasks: Option<bool>,
    pub limit: Option<i64>,
    pub page_id: Option<&'options [i64]>,
    pub space_id: Option<&'options [i64]>,
    pub status: Option<&'options str>,
    pub task_id: Option<&'options [i64]>,
}

impl crate::Operation for GetTasks {
    const BASE: crate::Base = crate::Base::Confluence;
    const IS_READ: bool = true;
    const METHOD: crate::Method = crate::Method::Get;
    const PATH: &'static str = "/wiki/api/v2/tasks";

    type Options<'options> = Options<'options>;
    type Reply = crate::Object<super::shape::GetTasksReply>;

    fn request(
        &self,
        options: &Self::Options<'_>,
        request: &mut crate::Request,
    ) -> Result<(), crate::Error> {
        assert!(request.is_blank(), "a request starts blank");

        request.query_texts("assigned-to", options.assigned_to);
        request.query_numbers("blogpost-id", options.blogpost_id);
        request.query_text("body-format", options.body_format);
        request.query_number("completed-at-from", options.completed_at_from);
        request.query_number("completed-at-to", options.completed_at_to);
        request.query_texts("completed-by", options.completed_by);
        request.query_number("created-at-from", options.created_at_from);
        request.query_number("created-at-to", options.created_at_to);
        request.query_texts("created-by", options.created_by);
        request.query_text("cursor", options.cursor);
        request.query_number("due-at-from", options.due_at_from);
        request.query_number("due-at-to", options.due_at_to);
        request.query_boolean("include-blank-tasks", options.include_blank_tasks);
        request.query_number("limit", options.limit);
        request.query_numbers("page-id", options.page_id);
        request.query_numbers("space-id", options.space_id);
        request.query_text("status", options.status);
        request.query_numbers("task-id", options.task_id);

        assert!(request.is_complete(), "every path parameter is filled");

        Ok(())
    }
}

impl crate::Paged for GetTasks {
    const PAGING: crate::Paging = crate::Paging {
        cursor: "cursor",
        items: "/results",
        next: crate::Next::Link("/_links/next"),
    };

    type Item = crate::Object<super::shape::Task>;
}
