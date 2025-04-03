mod context;
pub use context::*;

mod tree;
pub use tree::*;

pub trait GtjConvert<Node> {
    fn convert(&self, context: &mut GtjConvertContext) -> Node;
}
