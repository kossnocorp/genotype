use crate::prelude::internal::*;

pub trait PYConvert<Node> {
    fn convert(&self, context: &mut PYConvertContext) -> Node;
}
