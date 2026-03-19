use crate::prelude::internal::*;

pub fn convert_to_ts<GtNode: TSConvert<Node>, Node>(gt_node: GtNode) -> Node {
    gt_node.convert(&mut Default::default())
}

pub fn convert_to_ts_with<GtNode: TSConvert<Node>, Node, ContextFn>(
    gt_node: GtNode,
    context_fn: ContextFn,
) -> Node
where
    ContextFn: FnOnce(&mut TSConvertContext),
{
    let mut context = Default::default();
    context_fn(&mut context);
    gt_node.convert(&mut context)
}
