use crate::prelude::internal::*;

pub fn convert_node<GtNode: RSConvert<Node>, Node>(gt_node: GtNode) -> Node {
    gt_node.convert(&mut Default::default()).unwrap()
}

pub fn convert_node_with<GtNode: RSConvert<Node>, Node>(
    gt_node: GtNode,
    context: &mut RSConvertContext,
) -> Node {
    gt_node.convert(context).unwrap()
}
