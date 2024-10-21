use genotype_lang_py_tree::{tuple::PYTuple, PYContextResolve};
use genotype_parser::tree::tuple::GTTuple;

use crate::{context::PYConvertContext, convert::PYConvert};

impl PYConvert<PYTuple> for GTTuple {
    fn convert(&self, context: &mut PYConvertContext) -> PYTuple {
        PYTuple {
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
    use genotype_lang_py_config::{PYLangConfig, PYVersion};
    use genotype_lang_py_tree::*;
    use genotype_parser::tree::*;
    use pretty_assertions::assert_eq;

    use crate::resolve::PYConvertResolve;

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
            .convert(&mut PYConvertContext::default()),
            PYTuple {
                descriptors: vec![
                    PYDescriptor::Primitive(PYPrimitive::Boolean),
                    PYDescriptor::Primitive(PYPrimitive::String),
                ]
            }
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut context = PYConvertContext::new(
            PYConvertResolve::default(),
            PYLangConfig::new(PYVersion::Legacy),
        );
        assert_eq!(
            GTTuple {
                span: (0, 0).into(),
                descriptors: vec![GTPrimitive::String((0, 0).into()).into()],
            }
            .convert(&mut context),
            PYTuple {
                descriptors: vec![PYPrimitive::String.into()],
            }
        );
        assert_eq!(
            context.as_dependencies(),
            vec![(PYDependency::Typing, "Tuple".into())]
        );
    }
}
