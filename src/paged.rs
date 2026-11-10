use crate::kind::Kind;
use crate::operation::Operation;
use crate::paging::Paging;

pub trait Paged: Operation {
    const PAGING: Paging;

    type Item: Kind;
}
