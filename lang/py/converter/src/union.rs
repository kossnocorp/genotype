use genotype_lang_py_tree::{union::PYUnion, PYContextResolve};
use genotype_parser::tree::union::GTUnion;

use crate::{context::PYConvertContext, convert::PYConvert};

impl PYConvert<PYUnion> for GTUnion {
    fn convert(&self, context: &mut PYConvertContext) -> PYUnion {
        PYUnion {
            descriptors: self
                .descriptors
                .iter()
                .map(|descriptor| descriptor.convert(context))
                .collect(),
        }
        .resolve(&mut context.tree, &context.options)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use genotype_lang_py_tree::*;
    use genotype_parser::tree::*;
    use pretty_assertions::assert_eq;

    use crate::{context::PYConvertContext, mock::mock_context};

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            GTUnion {
                span: (0, 0).into(),
                descriptors: vec![
                    GTPrimitive::Boolean((0, 0).into()).into(),
                    GTPrimitive::String((0, 0).into()).into(),
                ]
            }
            .convert(&mut PYConvertContext::default()),
            PYUnion {
                descriptors: vec![
                    PYDescriptor::Primitive(PYPrimitive::Boolean),
                    PYDescriptor::Primitive(PYPrimitive::String),
                ]
            }
        );
    }

    #[test]
    fn test_convert_resolve() {
        let (_, context) = mock_context();
        let mut context = context;
        context.options.version = PYVersion::Legacy;
        assert_eq!(
            GTUnion {
                span: (0, 0).into(),
                descriptors: vec![GTPrimitive::String((0, 0).into()).into()],
            }
            .convert(&mut context),
            PYUnion {
                descriptors: vec![PYPrimitive::String.into()],
            }
        );
        assert_eq!(
            context.tree.imports,
            HashSet::from_iter(vec![("typing".into(), "Union".into())]),
        );
    }
}
