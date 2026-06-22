use crate::prelude::internal::*;

mod context;
pub use context::*;

mod error;
pub use error::*;

pub trait RsConvert<Node> {
    fn convert(&self, context: &mut RsConvertContext) -> RsConvertResult<Node>;
}

pub type RsConvertResult<Node> = Result<Node, RsConvertError>;
