#[derive(Debug)]
pub struct Response {
    pub body: Vec<u8>,
    pub is_near_limit: bool,
    pub location: Option<String>,
    pub rate_limit_reason: Option<String>,
    pub retry_after_seconds: Option<u64>,
    pub status: u16,
    pub streamed_count: u64,
}
