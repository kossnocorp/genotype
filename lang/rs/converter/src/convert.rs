use crate::prelude::internal::*;

pub trait RSConvert<Node> {
    fn convert(&self, context: &mut RSConvertContext) -> Result<Node>;
}
