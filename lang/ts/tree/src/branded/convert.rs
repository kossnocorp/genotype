use crate::prelude::internal::*;

impl TSConvert<TSBranded> for GTBranded {
    fn convert(&self, context: &mut TSConvertContext) -> TSBranded {
        let doc = context.consume_doc();
        let name = self.name.convert(context);
        let primitive = self.primitive.convert(context);

        TSBranded {
            doc,
            name,
            primitive,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            GTBranded {
                span: (0, 0).into(),
                id: GTDefinitionId("module".into(), "UserId".into()),
                name: GTIdentifier::new((0, 0).into(), "UserId".into()),
                primitive: GTPrimitive::String((0, 0).into()).into(),
            }
            .convert(&mut Default::default()),
            @r#"
        TSBranded(
          doc: None,
          name: TSIdentifier("UserId"),
          primitive: String,
        )
        "#
        );
    }

    #[test]
    fn test_doc() {
        let mut context = TSConvertContext::new(TSConvertResolve::new(), Default::default());
        context.provide_doc(Some("This is a user ID.".into()));
        assert_ron_snapshot!(
            GTBranded {
                span: (0, 0).into(),
                id: GTDefinitionId("module".into(), "UserId".into()),
                name: GTIdentifier::new((0, 0).into(), "UserId".into()),
                primitive: GTPrimitive::String((0, 0).into()).into(),
            }
            .convert(&mut context),
            @r#"
        TSBranded(
          doc: Some(TSDoc("This is a user ID.")),
          name: TSIdentifier("UserId"),
          primitive: String,
        )
        "#
        );
    }
}
