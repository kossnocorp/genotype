use genotype_lang_rs_tree::{union::RSUnion, RSContextResolve};
use genotype_parser::tree::union::GTUnion;

use crate::{context::RSConvertContext, convert::RSConvert};

impl RSConvert<RSUnion> for GTUnion {
    fn convert(&self, context: &mut RSConvertContext) -> RSUnion {
        RSUnion {
            descriptors: self
                .descriptors
                .iter()
                .map(|descriptor| descriptor.convert(context))
                .collect(),
            discriminator: None,
        }
        .resolve(context)
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_rs_config::{RSLangConfig, RSVersion};
    use genotype_lang_rs_tree::*;
    use genotype_parser::tree::*;
    use pretty_assertions::assert_eq;

    use crate::{context::RSConvertContext, resolve::RSConvertResolve};

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
            .convert(&mut RSConvertContext::default()),
            RSUnion {
                descriptors: vec![
                    RSDescriptor::Primitive(RSPrimitive::Boolean),
                    RSDescriptor::Primitive(RSPrimitive::String),
                ],
                discriminator: None
            }
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut context = RSConvertContext::new(
            RSConvertResolve::default(),
            RSLangConfig::new(RSVersion::Legacy),
        );
        assert_eq!(
            GTUnion {
                span: (0, 0).into(),
                descriptors: vec![GTPrimitive::String((0, 0).into()).into()],
            }
            .convert(&mut context),
            RSUnion {
                descriptors: vec![RSPrimitive::String.into()],
                discriminator: None
            }
        );
        assert_eq!(
            context.as_dependencies(),
            vec![(RSDependency::Typing, "Union".into())]
        );
    }
}
