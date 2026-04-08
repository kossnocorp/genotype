use crate::prelude::internal::*;

pub use genotype_test::*;

pub fn convert_node<GtNode: RsConvert<Node>, Node>(gt_node: GtNode) -> Node {
    gt_node.convert(&mut Default::default()).unwrap()
}

pub fn convert_node_with<GtNode: RsConvert<Node>, Node>(
    gt_node: GtNode,
    context: &mut RsConvertContext,
) -> Node {
    gt_node.convert(context).unwrap()
}

pub fn convert_node_err_with<GtNode: RsConvert<Node>, Node>(
    gt_node: GtNode,
    context: &mut RsConvertContext,
) -> miette::Report
where
    Node: std::fmt::Debug,
{
    let result = gt_node.convert(context);
    result.unwrap_err()
}

pub struct Rst {}

impl Rst {
    pub fn convert_context() -> RsConvertContext {
        RsConvertContext::empty("module".into())
    }

    pub fn convert_context_with_parent(parent_name: &str) -> RsConvertContext {
        let mut context = Self::convert_context();
        context.enter_parent(RsContextParent::Alias(parent_name.into()));
        context
    }

    pub fn convert_context_with_resolve(resolve: RsConvertResolve) -> RsConvertContext {
        RsConvertContext::new(
            "module".into(),
            resolve,
            Default::default(),
            Default::default(),
        )
    }
}
