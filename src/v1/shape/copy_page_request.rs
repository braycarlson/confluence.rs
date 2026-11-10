#[derive(Clone, Copy, Debug)]
pub struct CopyPageRequest;

impl CopyPageRequest {
    pub const BODY: crate::Field<Self, crate::Object<super::CopyPageRequestBody>> =
        crate::Field::new("body");

    pub const COPY_ATTACHMENTS: crate::Field<Self, crate::Boolean> =
        crate::Field::new("copyAttachments");

    pub const COPY_CUSTOM_CONTENTS: crate::Field<Self, crate::Boolean> =
        crate::Field::new("copyCustomContents");

    pub const COPY_LABELS: crate::Field<Self, crate::Boolean> = crate::Field::new("copyLabels");

    pub const COPY_PERMISSIONS: crate::Field<Self, crate::Boolean> =
        crate::Field::new("copyPermissions");

    pub const COPY_PROPERTIES: crate::Field<Self, crate::Boolean> =
        crate::Field::new("copyProperties");

    pub const DESTINATION: crate::Field<Self, crate::Object<super::CopyPageRequestDestination>> =
        crate::Field::new("destination");

    pub const PAGE_TITLE: crate::Field<Self, crate::Text> = crate::Field::new("pageTitle");
}
