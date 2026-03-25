use crate::prelude::internal::*;

pub trait PyConvert<Node> {
    fn convert(&self, context: &mut PyConvertContext) -> Node;
}
