use crate::prelude::internal::*;

pub fn convert_node<GtNode: TsConvert<Node>, Node>(gt_node: GtNode) -> Node {
    gt_node.convert(&mut Default::default())
}

pub fn convert_node_with<GtNode: TsConvert<Node>, Node>(
    gt_node: GtNode,
    context: &mut TsConvertContext,
) -> Node {
    gt_node.convert(context)
}
