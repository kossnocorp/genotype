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
        .resolve(context)
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_py_config::{PYLangConfig, PYVersion};
    use genotype_lang_py_tree::*;
    use pretty_assertions::assert_eq;

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
        let mut context =
            PYConvertContext::new(Default::default(), PYLangConfig::new(PYVersion::Legacy));
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
            context.as_dependencies(),
            vec![(PYDependency::Typing, "Optional".into())]
        );
    }
}
