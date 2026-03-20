use crate::prelude::internal::*;

pub fn convert_to_rs<GtNode: RSConvert<Node>, Node>(gt_node: GtNode) -> Node {
    gt_node.convert(&mut Default::default()).unwrap()
}

pub fn convert_to_rs_with<GtNode: RSConvert<Node>, Node, ContextFn>(
    gt_node: GtNode,
    context_fn: ContextFn,
) -> Node
where
    ContextFn: FnOnce(&mut RSConvertContext),
{
    let mut context = Default::default();
    context_fn(&mut context);
    gt_node.convert(&mut context).unwrap()
}

pub fn convert_to_rs_with_context<GtNode: RSConvert<Node>, Node>(
    gt_node: GtNode,
    context: &mut RSConvertContext,
) -> Node {
    gt_node.convert(context).unwrap()
}
