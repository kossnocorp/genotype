use crate::prelude::internal::*;

pub use genotype_test::*;

pub fn convert_node<GtNode: PyConvert<Node>, Node>(gt_node: GtNode) -> Node {
    gt_node.convert(&mut Default::default())
}

pub fn convert_node_with<GtNode: PyConvert<Node>, Node>(
    gt_node: GtNode,
    context: &mut PyConvertContext,
) -> Node {
    gt_node.convert(context)
}

pub struct Pyt {}

impl Pyt {
    pub fn convert_context() -> PyConvertContext {
        Default::default()
    }

    pub fn convert_context_legacy() -> PyConvertContext {
        PyConvertContext::new(
            Default::default(),
            PyConfig {
                lang: PyConfigLang::new(PyVersion::Legacy),
                ..Default::default()
            },
        )
    }
}
