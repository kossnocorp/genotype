use crate::prelude::internal::*;

pub fn convert_node<GtNode: TSConvert<Node>, Node>(gt_node: GtNode) -> Node {
    gt_node.convert(&mut Default::default())
}

pub fn convert_node_with<GtNode: TSConvert<Node>, Node>(
    gt_node: GtNode,
    context: &mut TSConvertContext,
) -> Node {
    gt_node.convert(context)
}
