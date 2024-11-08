use genotype_lang_rs_tree::{tuple::RSTuple, RSContextResolve};
use genotype_parser::tree::tuple::GTTuple;

use crate::{context::RSConvertContext, convert::RSConvert};

impl RSConvert<RSTuple> for GTTuple {
    fn convert(&self, context: &mut RSConvertContext) -> RSTuple {
        RSTuple {
            descriptors: self
                .descriptors
                .iter()
                .map(|descriptor| descriptor.convert(context))
                .collect(),
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

    use crate::resolve::RSConvertResolve;

    use super::*;

    #[test]
    fn test_convert() {
        assert_eq!(
            GTTuple {
                span: (0, 0).into(),
                descriptors: vec![
                    GTPrimitive::Boolean((0, 0).into()).into(),
                    GTPrimitive::String((0, 0).into()).into(),
                ]
            }
            .convert(&mut RSConvertContext::default()),
            RSTuple {
                descriptors: vec![
                    RSDescriptor::Primitive(RSPrimitive::Boolean),
                    RSDescriptor::Primitive(RSPrimitive::String),
                ]
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
            GTTuple {
                span: (0, 0).into(),
                descriptors: vec![GTPrimitive::String((0, 0).into()).into()],
            }
            .convert(&mut context),
            RSTuple {
                descriptors: vec![RSPrimitive::String.into()],
            }
        );
        assert_eq!(
            context.as_dependencies(),
            vec![(RSDependency::Typing, "Tuple".into())]
        );
    }
}
