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
    use insta::assert_ron_snapshot;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            GTBranded {
                span: (0, 0).into(),
                id: GTDefinitionId("module".into(), "UserId".into()),
                name: GTIdentifier::new((0, 0).into(), "UserId".into()),
                primitive: GTPrimitive::String((0, 0).into()).into(),
            }
            .convert(&mut PYConvertContext::default()),
            @r#"
        PYNewtype(
          doc: None,
          name: PYIdentifier("UserId"),
          primitive: String,
        )
        "#
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut context = PYConvertContext::default();
        assert_ron_snapshot!(
            GTBranded {
                span: (0, 0).into(),
                id: GTDefinitionId("module".into(), "UserId".into()),
                name: GTIdentifier::new((0, 0).into(), "UserId".into()),
                primitive: GTPrimitive::String((0, 0).into()).into(),
            }
            .convert(&mut context),
            @r#"
        PYNewtype(
          doc: None,
          name: PYIdentifier("UserId"),
          primitive: String,
        )
        "#
        );
        assert_ron_snapshot!(
            context.as_dependencies(),
            @r#"
        [
          (Typing, PYIdentifier("NewType")),
        ]
        "#
        );
    }

    #[test]
    fn test_convert_doc() {
        let mut context = PYConvertContext::default();
        context.provide_doc(Some(PYDoc("Hello, world!".into())));
        assert_ron_snapshot!(
            GTBranded {
                span: (0, 0).into(),
                id: GTDefinitionId("module".into(), "UserId".into()),
                name: GTIdentifier::new((0, 0).into(), "UserId".into()),
                primitive: GTPrimitive::String((0, 0).into()).into(),
            }
            .convert(&mut context),
            @r#"
        PYNewtype(
          doc: Some(PYDoc("Hello, world!")),
          name: PYIdentifier("UserId"),
          primitive: String,
        )
        "#
        );
    }
}
