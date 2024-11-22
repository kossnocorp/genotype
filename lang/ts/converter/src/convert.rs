use crate::context::TSConvertContext;

pub trait TSConvert<TSNode> {
    fn convert(&self, context: &mut TSConvertContext) -> TSNode;
}
