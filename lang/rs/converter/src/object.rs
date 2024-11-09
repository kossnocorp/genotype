use genotype_lang_rs_tree::*;
use genotype_parser::*;

use crate::{context::RSConvertContext, convert::RSConvert};

impl RSConvert<RSClass> for GTObject {
    fn convert(&self, context: &mut RSConvertContext) -> RSClass {
        context.create_references_scope();

        let name = match &self.name {
            GTObjectName::Named(identifier) => identifier.convert(context),
            GTObjectName::Alias(identifier, _) => identifier.convert(context),
            _ => panic!("Invalid object name"),
        };

        let doc = context.consume_doc();
        let extensions = self.extensions.iter().map(|e| e.convert(context)).collect();
        let properties = self.properties.iter().map(|p| p.convert(context)).collect();

        let references = context.pop_references_scope();

        RSClass {
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
    use genotype_lang_rs_tree::*;
    use genotype_parser::tree::*;
    use pretty_assertions::assert_eq;

    use super::*;

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
                        descriptor: GTPrimitive::Int((0, 0).into()).into(),
                        required: false,
                    }
                ]
            }
            .convert(&mut RSConvertContext::default()),
            RSClass {
                doc: None,
                name: "Person".into(),
                extensions: vec![],
                properties: vec![
                    RSProperty {
                        doc: None,
                        attributes: vec![],
                        name: "name".into(),
                        descriptor: RSDescriptor::Primitive(RSPrimitive::String).into(),
                    },
                    RSProperty {
                        doc: None,
                        attributes: vec![],
                        name: "age".into(),
                        descriptor: RSOption::new(RSDescriptor::Primitive(RSPrimitive::Int)).into(),
                    }
                ],
                references: vec![],
            }
        );
    }

    #[test]
    fn test_convert_resolve() {
        let mut context = RSConvertContext::default();
        assert_eq!(
            GTObject {
                span: (0, 0).into(),
                name: GTObjectName::Named(GTIdentifier::new((0, 0).into(), "Person".into())),
                extensions: vec![],
                properties: vec![]
            }
            .convert(&mut context),
            RSClass {
                doc: None,
                name: "Person".into(),
                extensions: vec![],
                properties: vec![],
                references: vec![],
            }
        );
        assert_eq!(
            context.as_dependencies(),
            vec![(RSDependency::Runtime, "Model".into())]
        );
    }

    #[test]
    fn test_convert_doc() {
        let mut context = RSConvertContext::default();
        context.provide_doc(Some(RSDoc("Hello, world!".into())));
        assert_eq!(
            GTObject {
                span: (0, 0).into(),
                name: GTObjectName::Named(GTIdentifier::new((0, 0).into(), "Person".into())),
                extensions: vec![],
                properties: vec![],
            }
            .convert(&mut context),
            RSClass {
                doc: Some(RSDoc("Hello, world!".into())),
                name: "Person".into(),
                extensions: vec![],
                properties: vec![],
                references: vec![],
            }
        );
    }
}
