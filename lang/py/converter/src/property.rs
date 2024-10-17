use genotype_lang_py_tree::{property::PYProperty, PYContextResolve};
use genotype_parser::tree::property::GTProperty;

use crate::{context::PYConvertContext, convert::PYConvert};

impl PYConvert<PYProperty> for GTProperty {
    fn convert(&self, context: &mut PYConvertContext) -> PYProperty {
        PYProperty {
            name: self.name.convert(context),
            descriptor: self.descriptor.convert(context),
            required: self.required,
        }
        .resolve(&mut context.tree, &context.options)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use genotype_lang_py_tree::*;
    use pretty_assertions::assert_eq;

    use crate::mock::mock_context;

    use super::*;
    use genotype_parser::tree::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            GTProperty {
                span: (0, 0).into(),
                doc: None,
                name: GTKey::new((0, 0).into(), "name".into()),
                descriptor: GTPrimitive::String((0, 0).into()).into(),
                required: false,
            }
            .convert(&mut PYConvertContext::default()),
            PYProperty {
                name: "name".into(),
                descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                required: false,
            }
        );
    }

    #[test]
    fn test_convert_resolve() {
        let (_, context) = mock_context();
        let mut context = context;
        context.options.version = PYVersion::Legacy;
        assert_eq!(
            GTProperty {
                doc: None,
                span: (0, 0).into(),
                name: GTKey::new((0, 0).into(), "name".into()),
                descriptor: GTPrimitive::String((0, 0).into()).into(),
                required: false,
            }
            .convert(&mut context),
            PYProperty {
                name: "name".into(),
                descriptor: PYPrimitive::String.into(),
                required: false,
            }
        );
        assert_eq!(
            context.tree.imports,
            HashSet::from_iter(vec![("typing".into(), "Optional".into())]),
        );
    }
}
