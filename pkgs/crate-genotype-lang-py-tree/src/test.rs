use crate::prelude::internal::*;

pub fn convert_node<GtNode: PyConvert<Node>, Node>(gt_node: GtNode) -> Node {
    gt_node.convert(&mut Default::default())
}

pub fn convert_node_with<GtNode: PyConvert<Node>, Node>(
    gt_node: GtNode,
    context: &mut PyConvertContext,
) -> Node {
    gt_node.convert(context)
}
