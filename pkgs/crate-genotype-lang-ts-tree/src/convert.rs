use crate::prelude::internal::*;

pub trait TsConvert<TsNode> {
    fn convert(&self, context: &mut TsConvertContext) -> TsNode;
}
