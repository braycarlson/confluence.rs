#[derive(Clone, Copy, Debug)]
pub struct Part<'part> {
    pub bytes: &'part [u8],
    pub content_type: &'part str,
    pub file_name: Option<&'part str>,
    pub name: &'part str,
}
