use genotype_lang_rs_tree::{property::RSProperty, RSContextResolve, RSOption};
use genotype_parser::tree::property::GTProperty;

use crate::{context::RSConvertContext, convert::RSConvert};

impl RSConvert<RSProperty> for GTProperty {
    fn convert(&self, context: &mut RSConvertContext) -> RSProperty {
        let descriptor = self.descriptor.convert(context);

        let descriptor = if self.required {
            descriptor
        } else {
            RSOption::new(descriptor).into()
        };

        RSProperty {
            doc: self.doc.as_ref().and_then(|doc| Some(doc.convert(context))),
            attributes: vec![],
            name: self.name.convert(context),
            descriptor,
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
                attributes: vec![],
                name: "name".into(),
                descriptor: RSOption::new(RSPrimitive::String.into()).into(),
            }
        );
    }

    #[test]
    // [TODO] Resolve test
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
                attributes: vec![],
                name: "name".into(),
                descriptor: RSOption::new(RSPrimitive::String.into()).into(),
            }
        );
        assert_eq!(context.as_dependencies(), vec![]);
    }
}