use crate::prelude::internal::*;

impl PYConvert<PYUnion> for GTUnion {
    fn convert(&self, context: &mut PYConvertContext) -> PYUnion {
        PYUnion {
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
    use super::*;
    use pretty_assertions::assert_eq;

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
                ],
                discriminator: None
            }
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut context = PYConvertContext::new(
            PYConvertResolve::default(),
            PyConfig {
                lang: PyConfigLang::new(PYVersion::Legacy),
                ..Default::default()
            },
        );
        assert_eq!(
            GTUnion {
                span: (0, 0).into(),
                descriptors: vec![GTPrimitive::String((0, 0).into()).into()],
            }
            .convert(&mut context),
            PYUnion {
                descriptors: vec![PYPrimitive::String.into()],
                discriminator: None
            }
        );
        assert_eq!(
            context.as_dependencies(),
            vec![(PYDependencyIdent::Typing, "Union".into())]
        );
    }
}
