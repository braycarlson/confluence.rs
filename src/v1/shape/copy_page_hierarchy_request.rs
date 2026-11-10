#[derive(Clone, Copy, Debug)]
pub struct CopyPageHierarchyRequest;

impl CopyPageHierarchyRequest {
    pub const COPY_ATTACHMENTS: crate::Field<Self, crate::Boolean> =
        crate::Field::new("copyAttachments");

    pub const COPY_CUSTOM_CONTENTS: crate::Field<Self, crate::Boolean> =
        crate::Field::new("copyCustomContents");

    pub const COPY_DESCENDANTS: crate::Field<Self, crate::Boolean> =
        crate::Field::new("copyDescendants");

    pub const COPY_LABELS: crate::Field<Self, crate::Boolean> = crate::Field::new("copyLabels");

    pub const COPY_PERMISSIONS: crate::Field<Self, crate::Boolean> =
        crate::Field::new("copyPermissions");

    pub const COPY_PROPERTIES: crate::Field<Self, crate::Boolean> =
        crate::Field::new("copyProperties");

    pub const DESTINATION_PAGE_ID: crate::Field<Self, crate::Text> =
        crate::Field::new("destinationPageId");

    pub const TITLE_OPTIONS: crate::Field<
        Self,
        crate::Object<super::CopyPageHierarchyTitleOptions>,
    > = crate::Field::new("titleOptions");
}
