pub trait GtjSchemaConvert<Node> {
    fn convert(&self, context: &mut GtjSchemaConvertContext) -> Node;
}

pub struct GtjSchemaConvertContext {}
