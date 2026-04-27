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

    pub fn convert_context_with(
        path_module_ids: Vec<(GtPathModuleId, GtModuleId)>,
        reference_definition_ids: Vec<(GtReferenceId, GtDefinitionId)>,
    ) -> RsConvertContext {
        RsConvertContext::new(
            "module".into(),
            Self::convert_resolve(path_module_ids, reference_definition_ids),
            Default::default(),
            Default::default(),
        )
    }

    pub fn convert_resolve(
        path_module_ids: Vec<(GtPathModuleId, GtModuleId)>,
        reference_definition_ids: Vec<(GtReferenceId, GtDefinitionId)>,
    ) -> RsConvertResolve {
        RsConvertResolve {
            path_module_ids: IndexMap::from_iter(path_module_ids),
            reference_definition_ids: IndexMap::from_iter(reference_definition_ids),
            ..Default::default()
        }
    }

    pub fn convert_context_with_resolve(resolve: RsConvertResolve) -> RsConvertContext {
        RsConvertContext::new(
            "module".into(),
            resolve,
            Default::default(),
            Default::default(),
        )
    }

    pub fn context_parent(name: &str) -> RsContextParent {
        RsContextParent::Alias(name.into())
    }
}
