use crate::context::RSConvertContext;
use miette::Result;

pub trait RSConvert<Node> {
    fn convert(&self, context: &mut RSConvertContext) -> Result<Node>;
}
