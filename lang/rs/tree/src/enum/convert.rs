use crate::prelude::internal::*;
use std::collections::HashSet;

impl RSConvert<RSEnum> for GTUnion {
    fn convert(&self, context: &mut RSConvertContext) -> Result<RSEnum> {
        let doc = context.consume_doc();
        let name = if let Some(name) = context.claim_alias() {
            name
        } else {
            context.name_child(None)
        };
        let id = context.build_definition_id(&name);
        context.drop_definition_id();
        context.enter_parent(RSContextParent::Definition(name.clone()));

        let mut variant_names: HashSet<RSIdentifier> = HashSet::new();

        let variants = self
            .descriptors
            .iter()
            .map(|descriptor| convert_variant(descriptor, &mut variant_names, context))
            .collect::<Result<Vec<_>>>()?;

        let r#enum = RSEnum {
            id,
            doc,
            name,
            attributes: vec![
                context
                    .render_derive(RSContextRenderDeriveMode::UnionEnum)
                    .into(),
                r#"serde(untagged)"#.into(),
            ],
            variants,
        };

        context.add_import(RSDependencyIdent::Serde, "Deserialize".into());
        context.add_import(RSDependencyIdent::Serde, "Serialize".into());

        context.exit_parent();
        Ok(r#enum)
    }
}

fn convert_variant(
    descriptor: &GTDescriptor,
    variant_names: &mut HashSet<RSIdentifier>,
    context: &mut RSConvertContext,
) -> Result<RSEnumVariant> {
    let name = name_descriptor(descriptor, context)?;
    let name = ensure_unique_name(name, variant_names);

    context.enter_parent(RSContextParent::EnumVariant(name.clone()));

    let descriptor = RSEnumVariantDescriptor::Descriptor(descriptor.convert(context)?);

    let enum_variant = RSEnumVariant {
        doc: None,
        attributes: vec![],
        name,
        descriptor,
    };

    context.exit_parent();
    Ok(enum_variant)
}

fn ensure_unique_name(
    name: RSIdentifier,
    variant_names: &mut HashSet<RSIdentifier>,
) -> RSIdentifier {
    let name = if !variant_names.contains(&name) {
        name
    } else {
        enumerated_name(&name, variant_names)
    };

    variant_names.insert(name.clone());

    name
}

fn enumerated_name(name: &RSIdentifier, variant_names: &HashSet<RSIdentifier>) -> RSIdentifier {
    let mut index = 2;
    loop {
        let enumerated_name = format!("{}{}", name.0, index).into();
        if !variant_names.contains(&enumerated_name) {
            return enumerated_name;
        }
        index += 1;
    }
}

fn name_descriptor(
    descriptor: &GTDescriptor,
    context: &mut RSConvertContext,
) -> Result<RSIdentifier> {
    Ok(match descriptor {
        GTDescriptor::Alias(alias) => alias.name.convert(context)?,
        GTDescriptor::Reference(reference) => reference.identifier.convert(context)?,
        GTDescriptor::InlineImport(import) => import.name.convert(context)?,
        GTDescriptor::Object(object) => object.name.to_identifier().convert(context)?,
        GTDescriptor::Literal(literal) => RSConvertNameSegment::Literal(literal.clone())
            .render(true)
            .into(),
        GTDescriptor::Branded(branded) => branded.name.convert(context)?,
        GTDescriptor::Primitive(primitive) => match primitive {
            GTPrimitive::Boolean(_) => "Boolean".into(),
            GTPrimitive::String(_) => "String".into(),
            GTPrimitive::Number(_) => "Number".into(),
            GTPrimitive::Int8(_) => "Int8".into(),
            GTPrimitive::Int16(_) => "Int16".into(),
            GTPrimitive::Int32(_) => "Int32".into(),
            GTPrimitive::Int64(_) => "Int".into(),
            GTPrimitive::Int128(_) => "Int128".into(),
            GTPrimitive::IntSize(_) => "IntSize".into(),
            GTPrimitive::IntU8(_) => "IntU8".into(),
            GTPrimitive::IntU16(_) => "IntU16".into(),
            GTPrimitive::IntU32(_) => "IntU32".into(),
            GTPrimitive::IntU64(_) => "IntU64".into(),
            GTPrimitive::IntU128(_) => "IntU128".into(),
            GTPrimitive::IntUSize(_) => "IntUSize".into(),
            GTPrimitive::Float32(_) => "Float32".into(),
            GTPrimitive::Float64(_) => "Float".into(),
            GTPrimitive::Null(_) => "Null".into(),
        },
        GTDescriptor::Array(_) => "Vec".into(),
        GTDescriptor::Union(_) => "Union".into(),
        GTDescriptor::Record(_) => "Map".into(),
        GTDescriptor::Tuple(_) => "Tuple".into(),
        GTDescriptor::Any(_) => "Any".into(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Alias("Union".into()));
        assert_eq!(
            GTUnion {
                span: (0, 0).into(),
                descriptors: vec![
                    GTPrimitive::Boolean((0, 0).into()).into(),
                    GTPrimitive::String((0, 0).into()).into(),
                ]
            }
            .convert(&mut context)
            .unwrap(),
            RSEnum {
                id: GTDefinitionId("module".into(), "Union".into()),
                doc: None,
                attributes: vec![
                    "derive(Debug, Clone, PartialEq, Serialize, Deserialize)".into(),
                    r#"serde(untagged)"#.into(),
                ],
                name: "Union".into(),
                variants: vec![
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "Boolean".into(),
                        descriptor: RSEnumVariantDescriptor::Descriptor(
                            RSPrimitive::Boolean.into()
                        ),
                    },
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "String".into(),
                        descriptor: RSEnumVariantDescriptor::Descriptor(RSPrimitive::String.into()),
                    }
                ],
            }
        );
    }

    #[test]
    fn test_convert_import() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Alias("Union".into()));
        assert_eq!(
            GTUnion {
                span: (0, 0).into(),
                descriptors: vec![GTPrimitive::String((0, 0).into()).into()],
            }
            .convert(&mut context)
            .unwrap(),
            RSEnum {
                id: GTDefinitionId("module".into(), "Union".into()),
                doc: None,
                attributes: vec![
                    "derive(Debug, Clone, PartialEq, Serialize, Deserialize)".into(),
                    r#"serde(untagged)"#.into(),
                ],
                name: "Union".into(),
                variants: vec![RSEnumVariant {
                    doc: None,
                    attributes: vec![],
                    name: "String".into(),
                    descriptor: RSEnumVariantDescriptor::Descriptor(RSPrimitive::String.into()),
                }],
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
    fn test_convert_doc() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Alias("Union".into()));
        context.provide_doc(Some("Hello, world!".into()));
        assert_eq!(
            GTUnion {
                span: (0, 0).into(),
                descriptors: vec![GTPrimitive::String((0, 0).into()).into()],
            }
            .convert(&mut context)
            .unwrap(),
            RSEnum {
                id: GTDefinitionId("module".into(), "Union".into()),
                doc: Some("Hello, world!".into()),
                attributes: vec![
                    "derive(Debug, Clone, PartialEq, Serialize, Deserialize)".into(),
                    r#"serde(untagged)"#.into(),
                ],
                name: "Union".into(),
                variants: vec![RSEnumVariant {
                    doc: None,
                    attributes: vec![],
                    name: "String".into(),
                    descriptor: RSEnumVariantDescriptor::Descriptor(RSPrimitive::String.into()),
                }],
            }
        );
    }

    #[test]
    fn test_naming() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Definition("Admin".into()));
        context.enter_parent(RSContextParent::Field("role".into()));
        assert_eq!(
            GTUnion {
                span: (0, 0).into(),
                descriptors: vec![
                    GTLiteral::String((0, 1).into(), "superadmin".into()).into(),
                    GTLiteral::String((0, 2).into(), "admin".into()).into(),
                    GTLiteral::String((0, 3).into(), "moderator".into()).into(),
                ],
            }
            .convert(&mut context)
            .unwrap(),
            RSEnum {
                id: GTDefinitionId("module".into(), "AdminRole".into()),
                doc: None,
                attributes: vec![
                    "derive(Debug, Clone, PartialEq, Serialize, Deserialize)".into(),
                    r#"serde(untagged)"#.into(),
                ],
                name: "AdminRole".into(),
                variants: vec![
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "Superadmin".into(),
                        descriptor: RSEnumVariantDescriptor::Descriptor(
                            RSReference {
                                id: GTReferenceId("module".into(), (0, 1).into()),
                                identifier: "AdminRoleSuperadmin".into(),
                                definition_id: GTDefinitionId(
                                    "module".into(),
                                    "AdminRoleSuperadmin".into()
                                )
                            }
                            .into()
                        ),
                    },
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "Admin".into(),
                        descriptor: RSEnumVariantDescriptor::Descriptor(
                            RSReference {
                                id: GTReferenceId("module".into(), (0, 2).into()),
                                identifier: "AdminRoleAdmin".into(),
                                definition_id: GTDefinitionId(
                                    "module".into(),
                                    "AdminRoleAdmin".into()
                                )
                            }
                            .into()
                        ),
                    },
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "Moderator".into(),
                        descriptor: RSEnumVariantDescriptor::Descriptor(
                            RSReference {
                                id: GTReferenceId("module".into(), (0, 3).into()),
                                identifier: "AdminRoleModerator".into(),
                                definition_id: GTDefinitionId(
                                    "module".into(),
                                    "AdminRoleModerator".into()
                                )
                            }
                            .into()
                        ),
                    },
                ],
            }
        );
    }

    #[test]
    fn test_unique_name() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Alias("Union".into()));
        assert_eq!(
            GTUnion {
                span: (0, 0).into(),
                descriptors: vec![
                    GTTuple {
                        span: (0, 0).into(),
                        descriptors: vec![],
                    }
                    .into(),
                    GTPrimitive::String((0, 0).into()).into(),
                    GTTuple {
                        span: (0, 0).into(),
                        descriptors: vec![],
                    }
                    .into(),
                    GTPrimitive::String((0, 0).into()).into(),
                    GTTuple {
                        span: (0, 0).into(),
                        descriptors: vec![],
                    }
                    .into()
                ],
            }
            .convert(&mut context)
            .unwrap(),
            RSEnum {
                id: GTDefinitionId("module".into(), "Union".into()),
                doc: None,
                attributes: vec![
                    "derive(Debug, Clone, PartialEq, Serialize, Deserialize)".into(),
                    r#"serde(untagged)"#.into(),
                ],
                name: "Union".into(),
                variants: vec![
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "Tuple".into(),
                        descriptor: RSEnumVariantDescriptor::Descriptor(
                            RSTuple {
                                descriptors: vec![]
                            }
                            .into()
                        ),
                    },
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "String".into(),
                        descriptor: RSEnumVariantDescriptor::Descriptor(RSPrimitive::String.into()),
                    },
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "Tuple2".into(),
                        descriptor: RSEnumVariantDescriptor::Descriptor(
                            RSTuple {
                                descriptors: vec![]
                            }
                            .into()
                        ),
                    },
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "String2".into(),
                        descriptor: RSEnumVariantDescriptor::Descriptor(RSPrimitive::String.into()),
                    },
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "Tuple3".into(),
                        descriptor: RSEnumVariantDescriptor::Descriptor(
                            RSTuple {
                                descriptors: vec![]
                            }
                            .into()
                        ),
                    },
                ],
            }
        );
    }

    #[test]
    fn test_numeric_name() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Alias("Union".into()));
        assert_eq!(
            GTUnion {
                span: (0, 0).into(),
                descriptors: vec![
                    GTPrimitive::Int32((0, 0).into()).into(),
                    GTPrimitive::Int64((0, 0).into()).into(),
                    GTPrimitive::IntSize((0, 0).into()).into(),
                    GTPrimitive::Float32((0, 0).into()).into(),
                    GTPrimitive::Float64((0, 0).into()).into(),
                ],
            }
            .convert(&mut context)
            .unwrap(),
            RSEnum {
                id: GTDefinitionId("module".into(), "Union".into()),
                doc: None,
                attributes: vec![
                    "derive(Debug, Clone, PartialEq, Serialize, Deserialize)".into(),
                    r#"serde(untagged)"#.into(),
                ],
                name: "Union".into(),
                variants: vec![
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "Int32".into(),
                        descriptor: RSEnumVariantDescriptor::Descriptor(RSPrimitive::Int32.into()),
                    },
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "Int".into(),
                        descriptor: RSEnumVariantDescriptor::Descriptor(RSPrimitive::Int64.into()),
                    },
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "IntSize".into(),
                        descriptor: RSEnumVariantDescriptor::Descriptor(
                            RSPrimitive::IntSize.into()
                        ),
                    },
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "Float32".into(),
                        descriptor: RSEnumVariantDescriptor::Descriptor(
                            RSPrimitive::Float32.into()
                        ),
                    },
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "Float".into(),
                        descriptor: RSEnumVariantDescriptor::Descriptor(
                            RSPrimitive::Float64.into()
                        ),
                    },
                ],
            }
        );
    }

    #[test]
    fn test_literal_name() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Alias("Union".into()));
        assert_eq!(
            GTUnion {
                span: (0, 0).into(),
                descriptors: vec![
                    GTLiteral::Null((0, 1).into()).into(),
                    GTLiteral::String((0, 2).into(), "Hello".into()).into(),
                    GTLiteral::Boolean((0, 3).into(), true).into(),
                ],
            }
            .convert(&mut context)
            .unwrap(),
            RSEnum {
                id: GTDefinitionId("module".into(), "Union".into()),
                doc: None,
                attributes: vec![
                    "derive(Debug, Clone, PartialEq, Serialize, Deserialize)".into(),
                    r#"serde(untagged)"#.into(),
                ],
                name: "Union".into(),
                variants: vec![
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "Null".into(),
                        descriptor: RSEnumVariantDescriptor::Descriptor(
                            RSReference {
                                id: GTReferenceId("module".into(), (0, 1).into()),
                                identifier: "UnionNull".into(),
                                definition_id: GTDefinitionId("module".into(), "UnionNull".into())
                            }
                            .into()
                        ),
                    },
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "Hello".into(),
                        descriptor: RSEnumVariantDescriptor::Descriptor(
                            RSReference {
                                id: GTReferenceId("module".into(), (0, 2).into()),
                                identifier: "UnionHello".into(),
                                definition_id: GTDefinitionId("module".into(), "UnionHello".into())
                            }
                            .into()
                        ),
                    },
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "True".into(),
                        descriptor: RSEnumVariantDescriptor::Descriptor(
                            RSReference {
                                id: GTReferenceId("module".into(), (0, 3).into()),
                                identifier: "UnionTrue".into(),
                                definition_id: GTDefinitionId("module".into(), "UnionTrue".into())
                            }
                            .into()
                        ),
                    },
                ]
            }
        );
    }

    #[test]
    fn test_literal_integer_name() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Alias("Version".into()));
        assert_eq!(
            GTUnion {
                span: (0, 0).into(),
                descriptors: vec![
                    GTLiteral::Integer((0, 1).into(), 0).into(),
                    GTLiteral::Integer((0, 2).into(), 1).into(),
                ],
            }
            .convert(&mut context)
            .unwrap(),
            RSEnum {
                id: GTDefinitionId("module".into(), "Version".into()),
                doc: None,
                attributes: vec![
                    "derive(Debug, Clone, PartialEq, Serialize, Deserialize)".into(),
                    r#"serde(untagged)"#.into(),
                ],
                name: "Version".into(),
                variants: vec![
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "Lit0".into(),
                        descriptor: RSEnumVariantDescriptor::Descriptor(
                            RSReference {
                                id: GTReferenceId("module".into(), (0, 1).into()),
                                identifier: "Version0".into(),
                                definition_id: GTDefinitionId("module".into(), "Version0".into())
                            }
                            .into()
                        ),
                    },
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "Lit1".into(),
                        descriptor: RSEnumVariantDescriptor::Descriptor(
                            RSReference {
                                id: GTReferenceId("module".into(), (0, 2).into()),
                                identifier: "Version1".into(),
                                definition_id: GTDefinitionId("module".into(), "Version1".into())
                            }
                            .into()
                        ),
                    },
                ]
            }
        );
    }

    #[test]
    fn test_literal_float_name() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Alias("Version".into()));
        assert_eq!(
            GTUnion {
                span: (0, 0).into(),
                descriptors: vec![
                    GTLiteral::Float((0, 1).into(), 1.2).into(),
                    GTLiteral::Float((0, 2).into(), 3.4).into(),
                ],
            }
            .convert(&mut context)
            .unwrap(),
            RSEnum {
                id: GTDefinitionId("module".into(), "Version".into()),
                doc: None,
                attributes: vec![
                    "derive(Debug, Clone, PartialEq, Serialize, Deserialize)".into(),
                    r#"serde(untagged)"#.into(),
                ],
                name: "Version".into(),
                variants: vec![
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "Lit1_2".into(),
                        descriptor: RSEnumVariantDescriptor::Descriptor(
                            RSReference {
                                id: GTReferenceId("module".into(), (0, 1).into()),
                                identifier: "Version1_2".into(),
                                definition_id: GTDefinitionId("module".into(), "Version1_2".into())
                            }
                            .into()
                        ),
                    },
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "Lit3_4".into(),
                        descriptor: RSEnumVariantDescriptor::Descriptor(
                            RSReference {
                                id: GTReferenceId("module".into(), (0, 2).into()),
                                identifier: "Version3_4".into(),
                                definition_id: GTDefinitionId("module".into(), "Version3_4".into())
                            }
                            .into()
                        ),
                    },
                ]
            }
        );
    }

    #[test]
    fn test_literal_invalid_string_name() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Alias("Version".into()));
        assert_eq!(
            GTUnion {
                span: (0, 0).into(),
                descriptors: vec![
                    GTLiteral::String((0, 1).into(), "0".into()).into(),
                    GTLiteral::String((0, 2).into(), "1".into()).into(),
                ],
            }
            .convert(&mut context)
            .unwrap(),
            RSEnum {
                id: GTDefinitionId("module".into(), "Version".into()),
                doc: None,
                attributes: vec![
                    "derive(Debug, Clone, PartialEq, Serialize, Deserialize)".into(),
                    r#"serde(untagged)"#.into(),
                ],
                name: "Version".into(),
                variants: vec![
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "Lit0".into(),
                        descriptor: RSEnumVariantDescriptor::Descriptor(
                            RSReference {
                                id: GTReferenceId("module".into(), (0, 1).into()),
                                identifier: "Version0".into(),
                                definition_id: GTDefinitionId("module".into(), "Version0".into())
                            }
                            .into()
                        ),
                    },
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "Lit1".into(),
                        descriptor: RSEnumVariantDescriptor::Descriptor(
                            RSReference {
                                id: GTReferenceId("module".into(), (0, 2).into()),
                                identifier: "Version1".into(),
                                definition_id: GTDefinitionId("module".into(), "Version1".into())
                            }
                            .into()
                        ),
                    },
                ]
            }
        );
    }
}
