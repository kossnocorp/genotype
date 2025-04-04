use super::GtjTreeConvertContext;

pub trait GtjTreeConvert<Node> {
    fn to_tree(&self, context: &mut GtjTreeConvertContext) -> Node;
}
