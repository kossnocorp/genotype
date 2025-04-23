use crate::prelude::internal::*;

impl PYConvert<PYNewtype> for GTBranded {
    fn convert(&self, context: &mut PYConvertContext) -> PYNewtype {
        let doc = context.consume_doc();
        let name = self.name.convert(context);
        let primitive = self.primitive.convert(context);

        PYNewtype {
            doc,
            name,
            primitive,
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
            GTBranded {
                span: (0, 0).into(),
                id: GTDefinitionId("module".into(), "UserId".into()),
                name: GTIdentifier::new((0, 0).into(), "UserId".into()),
                primitive: GTPrimitive::String((0, 0).into()).into(),
            }
            .convert(&mut PYConvertContext::default()),
            PYNewtype {
                doc: None,
                name: "UserId".into(),
                primitive: PYPrimitive::String,
            }
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut context = PYConvertContext::default();
        assert_eq!(
            GTBranded {
                span: (0, 0).into(),
                id: GTDefinitionId("module".into(), "UserId".into()),
                name: GTIdentifier::new((0, 0).into(), "UserId".into()),
                primitive: GTPrimitive::String((0, 0).into()).into(),
            }
            .convert(&mut context),
            PYNewtype {
                doc: None,
                name: "UserId".into(),
                primitive: PYPrimitive::String,
            }
        );
        assert_eq!(
            context.as_dependencies(),
            vec![(PYDependencyIdent::Typing, "NewType".into())]
        );
    }

    #[test]
    fn test_convert_doc() {
        let mut context = PYConvertContext::default();
        context.provide_doc(Some(PYDoc("Hello, world!".into())));
        assert_eq!(
            GTBranded {
                span: (0, 0).into(),
                id: GTDefinitionId("module".into(), "UserId".into()),
                name: GTIdentifier::new((0, 0).into(), "UserId".into()),
                primitive: GTPrimitive::String((0, 0).into()).into(),
            }
            .convert(&mut context),
            PYNewtype {
                doc: Some(PYDoc("Hello, world!".into())),
                name: "UserId".into(),
                primitive: PYPrimitive::String,
            }
        );
    }
}
