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
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        assert_eq!(
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
            PYClass {
                doc: None,
                name: "Person".into(),
                extensions: vec![],
                properties: vec![
                    PYProperty {
                        doc: None,
                        name: "name".into(),
                        descriptor: PYDescriptor::Primitive(PYPrimitive::String),
                        required: true,
                    },
                    PYProperty {
                        doc: None,
                        name: "age".into(),
                        descriptor: PYDescriptor::Primitive(PYPrimitive::Int),
                        required: false,
                    }
                ],
                references: vec![],
            }
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut context = PYConvertContext::default();
        assert_eq!(
            GTObject {
                span: (0, 0).into(),
                name: GTObjectName::Named(GTIdentifier::new((0, 0).into(), "Person".into())),
                extensions: vec![],
                properties: vec![]
            }
            .convert(&mut context),
            PYClass {
                doc: None,
                name: "Person".into(),
                extensions: vec![],
                properties: vec![],
                references: vec![],
            }
        );
        assert_eq!(
            context.as_dependencies(),
            vec![(PYDependencyIdent::Runtime, "Model".into())]
        );
    }

    #[test]
    fn test_convert_doc() {
        let mut context = PYConvertContext::default();
        context.provide_doc(Some(PYDoc("Hello, world!".into())));
        assert_eq!(
            GTObject {
                span: (0, 0).into(),
                name: GTObjectName::Named(GTIdentifier::new((0, 0).into(), "Person".into())),
                extensions: vec![],
                properties: vec![],
            }
            .convert(&mut context),
            PYClass {
                doc: Some(PYDoc("Hello, world!".into())),
                name: "Person".into(),
                extensions: vec![],
                properties: vec![],
                references: vec![],
            }
        );
    }
}
