use crate::prelude::internal::*;

pub trait TSConvert<TSNode> {
    fn convert(&self, context: &mut TSConvertContext) -> TSNode;
}
