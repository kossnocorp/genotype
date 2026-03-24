use crate::prelude::internal::*;

pub fn convert_node<GtNode: PYConvert<Node>, Node>(gt_node: GtNode) -> Node {
    gt_node.convert(&mut Default::default())
}

pub fn convert_node_with<GtNode: PYConvert<Node>, Node>(
    gt_node: GtNode,
    context: &mut PYConvertContext,
) -> Node {
    gt_node.convert(context)
}
