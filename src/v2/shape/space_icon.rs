#[derive(Clone, Copy, Debug)]
pub struct SpaceIcon;

impl SpaceIcon {
    pub const API_DOWNLOAD_LINK: crate::Field<Self, crate::Text> =
        crate::Field::new("apiDownloadLink");

    pub const PATH: crate::Field<Self, crate::Text> = crate::Field::new("path");
}
