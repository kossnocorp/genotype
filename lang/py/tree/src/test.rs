use crate::prelude::internal::*;

pub fn convert_to_py<GtNode: PYConvert<Node>, Node>(gt_node: GtNode) -> Node {
    gt_node.convert(&mut Default::default())
}

pub fn convert_to_py_with<GtNode: PYConvert<Node>, Node, ContextFn>(
    gt_node: GtNode,
    context_fn: ContextFn,
) -> Node
where
    ContextFn: FnOnce(&mut PYConvertContext),
{
    let mut context = Default::default();
    context_fn(&mut context);
    gt_node.convert(&mut context)
}

pub fn convert_to_py_with_context<GtNode: PYConvert<Node>, Node>(
    gt_node: GtNode,
    context: &mut PYConvertContext,
) -> Node {
    gt_node.convert(context)
}
