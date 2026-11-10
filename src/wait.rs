use core::time::Duration;

#[derive(Clone, Copy, Debug)]
pub struct Wait<'wait> {
    pub delay: Duration,
    pub note: Option<&'wait str>,
    pub reason: &'static str,
}
