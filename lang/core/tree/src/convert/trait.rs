use crate::*;
use miette::Result;

pub trait GtlConvert<Node>
where
    Self: Sized,
{
    type ConvertContext: GtlConvertContext;

    fn convert(node: Node, context: &mut Self::ConvertContext) -> Result<Self>;
}
