use crate::context::RSConvertContext;

pub trait RSConvert<RSNode> {
    fn convert(&self, context: &mut RSConvertContext) -> RSNode;
}
