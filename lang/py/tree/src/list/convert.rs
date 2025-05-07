use crate::prelude::internal::*;

impl PYConvert<PYList> for GTArray {
    fn convert(&self, context: &mut PYConvertContext) -> PYList {
        PYList {
            descriptor: self.descriptor.convert(context),
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
        let mut context = PYConvertContext::new(
            Default::default(),
            PyConfigLang::new(PYVersion::Legacy),
            Default::default(),
        );
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
            context.as_dependencies(),
            vec![(PYDependencyIdent::Typing, "List".into())]
        );
    }
}
