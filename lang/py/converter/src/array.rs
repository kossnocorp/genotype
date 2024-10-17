use genotype_lang_py_tree::*;
use genotype_parser::tree::array::GTArray;

use crate::{context::PYConvertContext, convert::PYConvert};

impl PYConvert<PYList> for GTArray {
    fn convert(&self, context: &mut PYConvertContext) -> PYList {
        PYList {
            descriptor: self.descriptor.convert(context),
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
            GTArray {
                span: (0, 0).into(),
                descriptor: GTPrimitive::Boolean((0, 0).into()).into(),
            }
            .convert(&mut PYConvertContext::default()),
            PYList {
                descriptor: PYDescriptor::Primitive(PYPrimitive::Boolean)
            }
        );
    }

    #[test]
    fn test_convert_resolve() {
        let (_, context) = mock_context();
        let mut context = context;
        context.options.version = PYVersion::Legacy;
        assert_eq!(
            GTArray {
                span: (0, 0).into(),
                descriptor: GTPrimitive::String((0, 0).into()).into(),
            }
            .convert(&mut context),
            PYList {
                descriptor: PYPrimitive::String.into(),
            }
        );
        assert_eq!(
            context.tree.imports,
            HashSet::from_iter(vec![("typing".into(), "List".into())]),
        );
    }
}
