use crate::prelude::internal::*;

impl PYConvert<PYClass> for GTObject {
    fn convert(&self, context: &mut PYConvertContext) -> PYClass {
        context.create_references_scope();

        let name = match &self.name {
            GTObjectName::Named(identifier) => identifier.convert(context),
            GTObjectName::Alias(identifier, _) => identifier.convert(context),
        };

        let doc = context.consume_doc();
        let extensions = self.extensions.iter().map(|e| e.convert(context)).collect();
        let properties = self.properties.iter().map(|p| p.convert(context)).collect();

        let references = context.pop_references_scope();

        PYClass {
            doc,
            name,
            extensions,
            properties,
            references,
        }
        .resolve(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            GTObject {
                span: (0, 0).into(),
                name: GTObjectName::Named(GTIdentifier::new((0, 0).into(), "Person".into())),
                extensions: vec![],
                properties: vec![
                    GTProperty {
                        span: (0, 0).into(),
                        doc: None,
                        attributes: vec![],
                        name: GTKey::new((0, 0).into(), "name".into()),
                        descriptor: GTPrimitive::String((0, 0).into()).into(),
                        required: true,
                    },
                    GTProperty {
                        span: (0, 0).into(),
                        doc: None,
                        attributes: vec![],
                        name: GTKey::new((0, 0).into(), "age".into()),
                        descriptor: GTPrimitive::Int32((0, 0).into()).into(),
                        required: false,
                    }
                ]
            }
            .convert(&mut PYConvertContext::default()),
            @r#"
        PYClass(
          doc: None,
          name: PYIdentifier("Person"),
          extensions: [],
          properties: [
            PYProperty(
              doc: None,
              name: PYKey("name"),
              descriptor: Primitive(String),
              required: true,
            ),
            PYProperty(
              doc: None,
              name: PYKey("age"),
              descriptor: Primitive(Int),
              required: false,
            ),
          ],
          references: [],
        )
        "#
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut context = PYConvertContext::default();
        assert_ron_snapshot!(
            GTObject {
                span: (0, 0).into(),
                name: GTObjectName::Named(GTIdentifier::new((0, 0).into(), "Person".into())),
                extensions: vec![],
                properties: vec![]
            }
            .convert(&mut context),
            @r#"
        PYClass(
          doc: None,
          name: PYIdentifier("Person"),
          extensions: [],
          properties: [],
          references: [],
        )
        "#
        );
        assert_ron_snapshot!(
            context.as_dependencies(),
            @r#"
        [
          (Runtime, PYIdentifier("Model")),
        ]
        "#
        );
    }

    #[test]
    fn test_convert_doc() {
        let mut context = PYConvertContext::default();
        context.provide_doc(Some(PYDoc("Hello, world!".into())));
        assert_ron_snapshot!(
            GTObject {
                span: (0, 0).into(),
                name: GTObjectName::Named(GTIdentifier::new((0, 0).into(), "Person".into())),
                extensions: vec![],
                properties: vec![],
            }
            .convert(&mut context),
            @r#"
        PYClass(
          doc: Some(PYDoc("Hello, world!")),
          name: PYIdentifier("Person"),
          extensions: [],
          properties: [],
          references: [],
        )
        "#
        );
    }
}
