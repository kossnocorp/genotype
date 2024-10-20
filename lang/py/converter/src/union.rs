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
        .resolve(context)
    }
}

#[cfg(test)]
mod tests {
    use genotype_config::GTConfig;
    use genotype_lang_py_config::{PYConfig, PYVersion};
    use genotype_lang_py_tree::*;
    use genotype_parser::tree::*;
    use pretty_assertions::assert_eq;

    use crate::{context::PYConvertContext, resolve::PYConvertResolve};

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
        let mut context = PYConvertContext::new(
            PYConvertResolve::default(),
            (*GTConfig::default().with_python(PYConfig::new(PYVersion::Legacy))).clone(),
        );
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
            context.as_dependencies(),
            vec![(PYDependency::Typing, "Union".into())]
        );
    }
}
