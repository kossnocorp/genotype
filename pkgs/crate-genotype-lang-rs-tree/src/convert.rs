use crate::prelude::internal::*;

pub trait RsConvert<Node> {
    fn convert(&self, context: &mut RsConvertContext) -> Result<Node>;
}
