use crate::prelude::internal::*;

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

        context.add_import(RSDependencyIdent::Serde, "Deserialize".into());
        context.add_import(RSDependencyIdent::Serde, "Serialize".into());

        context.exit_parent();
        Ok(r#struct)
    }
}

impl RSConvert<RSStruct> for GTLiteral {
    fn convert(&self, context: &mut RSConvertContext) -> Result<RSStruct> {
        context.add_import(RSDependencyIdent::Literals, "literal".into());

        let doc = context.consume_doc();
        let name = if let Some(name) = context.claim_alias() {
            name
        } else {
            context.name_child(Some(&self.to_string()))
        };
        let id = context
            .consume_definition_id()
            .unwrap_or_else(|| context.build_definition_id(&name));

        let literal = render_literal(self);

        Ok(RSStruct {
            id,
            doc,
            attributes: vec![RSAttribute(format!("literal({literal})"))],
            name,
            fields: vec![].into(),
        })
    }
}

fn render_literal(literal: &GTLiteral) -> String {
    match literal {
        GTLiteral::Null(_) => "null".to_string(),
        GTLiteral::Boolean(_, value) => value.to_string(),
        GTLiteral::Integer(_, value) => value.to_string(),
        GTLiteral::Float(_, value) => GTLiteral::render_float(&value),
        GTLiteral::String(_, value) => GTLiteral::render_string(&value),
    }
}

impl RSConvert<RSStruct> for GTBranded {
    fn convert(&self, context: &mut RSConvertContext) -> Result<RSStruct> {
        let doc = context.consume_doc();
        let name = self.name.convert(context)?;
        let id = context
            .consume_definition_id()
            .unwrap_or_else(|| context.build_definition_id(&name));
        let descriptor = self.primitive.convert(context)?.into();

        Ok(RSStruct {
            id,
            doc,
            attributes: vec![context
                .render_derive(RSContextRenderDeriveMode::Struct)
                .into()],
            name,
            fields: RSStructFields::Tuple(vec![descriptor]),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert_object() {
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
    fn test_convert_object_import() {
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
                (RSDependencyIdent::Serde, "Deserialize".into()),
                (RSDependencyIdent::Serde, "Serialize".into())
            ]
        );
    }

    #[test]
    fn test_convert_object_doc() {
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
    fn test_convert_object_unresolved() {
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

    #[test]
    fn test_convert_literal() {
        assert_eq!(
            GTLiteral::Boolean((0, 0).into(), true)
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            RSStruct {
                id: GTDefinitionId("module".into(), "True".into()),
                doc: None,
                attributes: vec![RSAttribute("literal(true)".into())],
                name: "True".into(),
                fields: RSStructFields::Resolved(vec![])
            },
        );
    }

    #[test]
    fn test_convert_literal_name_from_alias() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Alias("Version".into()));
        assert_eq!(
            GTLiteral::Integer((0, 0).into(), 1)
                .convert(&mut context)
                .unwrap(),
            RSStruct {
                id: GTDefinitionId("module".into(), "Version".into()),
                doc: None,
                attributes: vec![RSAttribute("literal(1)".into())],
                name: "Version".into(),
                fields: RSStructFields::Resolved(vec![])
            },
        );
    }

    #[test]
    fn test_convert_literal_name_from_parents() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Definition("User".into()));
        context.enter_parent(RSContextParent::Field("v".into()));
        assert_eq!(
            GTLiteral::Integer((0, 0).into(), 1)
                .convert(&mut context)
                .unwrap(),
            RSStruct {
                id: GTDefinitionId("module".into(), "UserV1".into()),
                doc: None,
                attributes: vec![RSAttribute("literal(1)".into())],
                name: "UserV1".into(),
                fields: RSStructFields::Resolved(vec![])
            },
        );
    }

    #[test]
    fn test_convert_literal_import() {
        let mut context = RSConvertContext::empty("module".into());
        assert_eq!(
            GTLiteral::Boolean((0, 0).into(), false)
                .convert(&mut context)
                .unwrap(),
            RSStruct {
                id: GTDefinitionId("module".into(), "False".into()),
                doc: None,
                attributes: vec![RSAttribute("literal(false)".into())],
                name: "False".into(),
                fields: RSStructFields::Resolved(vec![])
            },
        );
        assert_eq!(
            context.as_dependencies(),
            vec![(RSDependencyIdent::Literals, "literal".into())]
        );
    }

    #[test]
    fn test_convert_literal_doc() {
        let mut context = RSConvertContext::empty("module".into());
        context.provide_doc(Some("Hello, world!".into()));
        assert_eq!(
            GTLiteral::Boolean((0, 0).into(), false)
                .convert(&mut context)
                .unwrap(),
            RSStruct {
                id: GTDefinitionId("module".into(), "False".into()),
                doc: Some("Hello, world!".into()),
                attributes: vec![RSAttribute("literal(false)".into())],
                name: "False".into(),
                fields: RSStructFields::Resolved(vec![])
            },
        );
    }

    #[test]
    fn test_convert_branded() {
        assert_eq!(
            GTLiteral::Boolean((0, 0).into(), true)
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            RSStruct {
                id: GTDefinitionId("module".into(), "True".into()),
                doc: None,
                attributes: vec![RSAttribute("literal(true)".into())],
                name: "True".into(),
                fields: RSStructFields::Resolved(vec![])
            },
        );
    }

    #[test]
    fn test_convert_branded_name_from_alias() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Alias("Version".into()));
        assert_eq!(
            GTLiteral::Integer((0, 0).into(), 1)
                .convert(&mut context)
                .unwrap(),
            RSStruct {
                id: GTDefinitionId("module".into(), "Version".into()),
                doc: None,
                attributes: vec![RSAttribute("literal(1)".into())],
                name: "Version".into(),
                fields: RSStructFields::Resolved(vec![])
            },
        );
    }

    #[test]
    fn test_convert_branded_name_from_parents() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Definition("User".into()));
        context.enter_parent(RSContextParent::Field("v".into()));
        assert_eq!(
            GTLiteral::Integer((0, 0).into(), 1)
                .convert(&mut context)
                .unwrap(),
            RSStruct {
                id: GTDefinitionId("module".into(), "UserV1".into()),
                doc: None,
                attributes: vec![RSAttribute("literal(1)".into())],
                name: "UserV1".into(),
                fields: RSStructFields::Resolved(vec![])
            },
        );
    }

    #[test]
    fn test_convert_branded_import() {
        let mut context = RSConvertContext::empty("module".into());
        assert_eq!(
            GTLiteral::Boolean((0, 0).into(), false)
                .convert(&mut context)
                .unwrap(),
            RSStruct {
                id: GTDefinitionId("module".into(), "False".into()),
                doc: None,
                attributes: vec![RSAttribute("literal(false)".into())],
                name: "False".into(),
                fields: RSStructFields::Resolved(vec![])
            },
        );
        assert_eq!(
            context.as_dependencies(),
            vec![(RSDependencyIdent::Literals, "literal".into())]
        );
    }

    #[test]
    fn test_convert_branded_doc() {
        let mut context = RSConvertContext::empty("module".into());
        context.provide_doc(Some("Hello, world!".into()));
        assert_eq!(
            GTLiteral::Boolean((0, 0).into(), false)
                .convert(&mut context)
                .unwrap(),
            RSStruct {
                id: GTDefinitionId("module".into(), "False".into()),
                doc: Some("Hello, world!".into()),
                attributes: vec![RSAttribute("literal(false)".into())],
                name: "False".into(),
                fields: RSStructFields::Resolved(vec![])
            },
        );
    }
}
