use crate::context::PYConvertContext;

pub trait PYConvert<PYNode> {
    fn convert(&self, context: &mut PYConvertContext) -> PYNode;
}
