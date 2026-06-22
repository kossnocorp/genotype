use crate::prelude::internal::*;

mod context;
pub use context::*;

pub trait GtlConvert<Node>
where
    Self: Sized,
{
    type ConvertContext: GtlConvertContext;

    fn convert(node: Node, context: &mut Self::ConvertContext) -> Result<Self>;
}
