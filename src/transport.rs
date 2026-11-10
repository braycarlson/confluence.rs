use std::io::{Seek, Write};

use crate::error::Error;
use crate::request::Request;
use crate::response::Response;
use crate::wait::Wait;

pub trait Transport: Sync {
    fn fetch(&self, request: &Request, bytes_max: u64) -> Result<Response, Error>;

    fn stream<Sink>(&self, request: &Request, sink: &mut Sink) -> Result<Response, Error>
    where
        Sink: Seek + Write;

    fn wait(&self, wait: &Wait<'_>) -> Result<(), Error>;
}
