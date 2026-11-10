use core::fmt;

use crate::base::Base;
use crate::error::Error;
use crate::method::Method;
use crate::request::Request;

pub trait Operation: fmt::Debug {
    const BASE: Base;
    const IS_READ: bool;
    const METHOD: Method;
    const PATH: &'static str;

    type Options<'options>: Copy + fmt::Debug + Default;
    type Reply;

    fn request(&self, options: &Self::Options<'_>, request: &mut Request) -> Result<(), Error>;
}
