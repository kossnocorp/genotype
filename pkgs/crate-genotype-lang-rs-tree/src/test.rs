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

pub fn convert_node_err_with<GtNode: RSConvert<Node>, Node>(
    gt_node: GtNode,
    context: &mut RSConvertContext,
) -> miette::Report
where
    Node: std::fmt::Debug,
{
    let result = gt_node.convert(context);
    result.unwrap_err()
}

pub struct Gtrs {}

impl Gtrs {
    pub fn convert_context() -> RSConvertContext {
        RSConvertContext::empty("module".into())
    }

    pub fn convert_context_with_parent(parent_name: &str) -> RSConvertContext {
        let mut context = Self::convert_context();
        context.enter_parent(RSContextParent::Alias(parent_name.into()));
        context
    }
}
