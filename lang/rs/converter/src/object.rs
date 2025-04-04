use genotype_lang_rs_tree::*;
use genotype_parser::*;
use miette::Result;

use crate::{
    context::{naming::RSContextParent, RSConvertContext},
    convert::RSConvert,
};

impl RSConvert<RSStruct> for GTObject {
    fn convert(&self, context: &mut RSConvertContext) -> Result<RSStruct> {
        let name = match &self.name {
            GTObjectName::Named(identifier) => identifier.convert(context),
            GTObjectName::Alias(identifier, _) => identifier.convert(context),
        }?;
        let id = context
            .consume_definition_id()
            .unwrap_or_else(|| context.build_definition_id(&name));
        context.enter_parent(RSContextParent::Definition(name.clone()));

        let doc = context.consume_doc();
        let fields = self
            .properties
            .iter()
            .map(|p| p.convert(context))
            .collect::<Result<Vec<_>>>()?;

        let fields = if self.extensions.len() > 0 {
            let references = self
                .extensions
                .iter()
                .map(|e| e.reference.convert(context))
                .collect::<Result<Vec<_>>>()?;
            RSStructFields::Unresolved(self.span.clone(), references, fields)
        } else {
            RSStructFields::Resolved(fields)
        };

        let r#struct = RSStruct {
            id,
            doc,
            attributes: vec![context
                .render_derive(RSContextRenderDeriveMode::Struct)
                .into()],
            name,
            fields,
        };

        context.import(RSDependency::Serde, "Deserialize".into());
        context.import(RSDependency::Serde, "Serialize".into());

        context.exit_parent();
        Ok(r#struct)
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
                        descriptor: GTPrimitive::Int32((0, 0).into()).into(),
                        required: false,
                    }
                ]
            }
            .convert(&mut RSConvertContext::empty("module".into()))
            .unwrap(),
            RSStruct {
                id: GTDefinitionId("module".into(), "Person".into()),
                doc: None,
                attributes: vec!["derive(Debug, Clone, PartialEq, Serialize, Deserialize)".into()],
                name: "Person".into(),
                fields: vec![
                    RSField {
                        doc: None,
                        attributes: vec![],
                        name: "name".into(),
                        descriptor: RSDescriptor::Primitive(RSPrimitive::String).into(),
                    },
                    RSField {
                        doc: None,
                        attributes: vec![],
                        name: "age".into(),
                        descriptor: RSOption::new(RSDescriptor::Primitive(RSPrimitive::Int32))
                            .into(),
                    }
                ]
                .into(),
            }
        );
    }

    #[test]
    fn test_convert_import() {
        let mut context = RSConvertContext::empty("module".into());
        assert_eq!(
            GTObject {
                span: (0, 0).into(),
                name: GTObjectName::Named(GTIdentifier::new((0, 0).into(), "Person".into())),
                extensions: vec![],
                properties: vec![]
            }
            .convert(&mut context)
            .unwrap(),
            RSStruct {
                id: GTDefinitionId("module".into(), "Person".into()),
                doc: None,
                attributes: vec!["derive(Debug, Clone, PartialEq, Serialize, Deserialize)".into()],
                name: "Person".into(),
                fields: vec![].into(),
            }
        );
        assert_eq!(
            context.as_dependencies(),
            vec![
                (RSDependency::Serde, "Deserialize".into()),
                (RSDependency::Serde, "Serialize".into())
            ]
        );
    }

    #[test]
    fn test_convert_doc() {
        let mut context = RSConvertContext::empty("module".into());
        context.provide_doc(Some("Hello, world!".into()));
        assert_eq!(
            GTObject {
                span: (0, 0).into(),
                name: GTObjectName::Named(GTIdentifier::new((0, 0).into(), "Person".into())),
                extensions: vec![],
                properties: vec![],
            }
            .convert(&mut context)
            .unwrap(),
            RSStruct {
                id: GTDefinitionId("module".into(), "Person".into()),
                doc: Some("Hello, world!".into()),
                attributes: vec!["derive(Debug, Clone, PartialEq, Serialize, Deserialize)".into()],
                name: "Person".into(),
                fields: vec![].into(),
            }
        );
    }

    #[test]
    fn test_convert_unresolved() {
        let mut context = RSConvertContext::empty("module".into());
        assert_eq!(
            GTObject {
                span: (1, 8).into(),
                name: GTObjectName::Named(GTIdentifier::new((0, 0).into(), "Person".into())),
                extensions: vec![GTExtension {
                    span: (0, 0).into(),
                    reference: GTReference {
                        span: (2, 9).into(),
                        id: GTReferenceId("module".into(), (2, 9).into()),
                        definition_id: GTReferenceDefinitionId::Resolved(GTDefinitionId(
                            "module".into(),
                            "Model".into()
                        )),
                        identifier: GTIdentifier::new((0, 0).into(), "Model".into())
                    }
                    .into(),
                }],
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
                        descriptor: GTPrimitive::IntSize((0, 0).into()).into(),
                        required: false,
                    }
                ]
            }
            .convert(&mut context)
            .unwrap(),
            RSStruct {
                id: GTDefinitionId("module".into(), "Person".into()),
                doc: None,
                attributes: vec!["derive(Debug, Clone, PartialEq, Serialize, Deserialize)".into()],
                name: "Person".into(),
                fields: RSStructFields::Unresolved(
                    (1, 8).into(),
                    vec![RSReference {
                        id: GTReferenceId("module".into(), (2, 9).into()),
                        identifier: "Model".into(),
                        definition_id: GTDefinitionId("module".into(), "Model".into())
                    }],
                    vec![
                        RSField {
                            doc: None,
                            attributes: vec![],
                            name: "name".into(),
                            descriptor: RSDescriptor::Primitive(RSPrimitive::String).into(),
                        },
                        RSField {
                            doc: None,
                            attributes: vec![],
                            name: "age".into(),
                            descriptor: RSOption::new(RSDescriptor::Primitive(
                                RSPrimitive::IntSize
                            ))
                            .into(),
                        }
                    ]
                )
            }
        );
    }
}
