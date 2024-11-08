use genotype_lang_rs_tree::{property::RSProperty, RSContextResolve};
use genotype_parser::tree::property::GTProperty;

use crate::{context::RSConvertContext, convert::RSConvert};

impl RSConvert<RSProperty> for GTProperty {
    fn convert(&self, context: &mut RSConvertContext) -> RSProperty {
        RSProperty {
            doc: self.doc.as_ref().and_then(|doc| Some(doc.convert(context))),
            name: self.name.convert(context),
            descriptor: self.descriptor.convert(context),
            required: self.required,
        }
        .resolve(context)
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_rs_config::{RSLangConfig, RSVersion};
    use genotype_lang_rs_tree::*;
    use pretty_assertions::assert_eq;

    use super::*;
    use genotype_parser::tree::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            GTProperty {
                span: (0, 0).into(),
                doc: None,
                attributes: vec![],
                name: GTKey::new((0, 0).into(), "name".into()),
                descriptor: GTPrimitive::String((0, 0).into()).into(),
                required: false,
            }
            .convert(&mut RSConvertContext::default()),
            RSProperty {
                doc: None,
                name: "name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                required: false,
            }
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut context =
            RSConvertContext::new(Default::default(), RSLangConfig::new(RSVersion::Legacy));
        assert_eq!(
            GTProperty {
                doc: None,
                span: (0, 0).into(),
                attributes: vec![],
                name: GTKey::new((0, 0).into(), "name".into()),
                descriptor: GTPrimitive::String((0, 0).into()).into(),
                required: false,
            }
            .convert(&mut context),
            RSProperty {
                doc: None,
                name: "name".into(),
                descriptor: RSPrimitive::String.into(),
                required: false,
            }
        );
        assert_eq!(
            context.as_dependencies(),
            vec![(RSDependency::Typing, "Optional".into())]
        );
    }
}
