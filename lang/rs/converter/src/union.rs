use std::collections::HashSet;

use genotype_lang_rs_tree::{
    RSContext, RSDependency, RSEnum, RSEnumVariant, RSEnumVariantDescriptor, RSIdentifier,
};
use genotype_parser::{tree::union::GTUnion, GTDescriptor, GTPrimitive};
use heck::ToPascalCase;
use miette::Result;

use crate::{
    context::{naming::RSContextParent, RSConvertContext},
    convert::RSConvert,
};

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
            attributes: vec![context.render_derive().into(), r#"serde(untagged)"#.into()],
            variants,
        };

        context.import(RSDependency::Serde, "Deserialize".into());
        context.import(RSDependency::Serde, "Serialize".into());

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
        GTDescriptor::Reference(reference) => reference.2.convert(context)?,
        GTDescriptor::InlineImport(import) => import.name.convert(context)?,
        GTDescriptor::Object(object) => object.name.to_identifier().convert(context)?,
        GTDescriptor::Literal(literal) => literal.to_string().to_pascal_case().into(),
        GTDescriptor::Primitive(primitive) => match primitive {
            GTPrimitive::Boolean(_) => "Boolean".into(),
            GTPrimitive::Float(_) => "Float".into(),
            GTPrimitive::Int(_) => "Int".into(),
            GTPrimitive::String(_) => "String".into(),
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
    use genotype_lang_rs_tree::*;
    use genotype_parser::tree::*;
    use pretty_assertions::assert_eq;

    use crate::context::RSConvertContext;

    use super::*;

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
                    "derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)"
                        .into(),
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
                    "derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)"
                        .into(),
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
                (RSDependency::Serde, "Deserialize".into()),
                (RSDependency::Serde, "Serialize".into())
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
                    "derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)"
                        .into(),
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
        context.enter_parent(RSContextParent::Property("role".into()));
        assert_eq!(
            GTUnion {
                span: (0, 0).into(),
                descriptors: vec![
                    GTLiteral::String((0, 0).into(), "superadmin".into()).into(),
                    GTLiteral::String((0, 0).into(), "admin".into()).into(),
                    GTLiteral::String((0, 0).into(), "moderator".into()).into(),
                ],
            }
            .convert(&mut context)
            .unwrap(),
            RSEnum {
                id: GTDefinitionId("module".into(), "AdminRole".into()),
                doc: None,
                attributes: vec![
                    "derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)"
                        .into(),
                    r#"serde(untagged)"#.into(),
                ],
                name: "AdminRole".into(),
                variants: vec![
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "Superadmin".into(),
                        descriptor: RSEnumVariantDescriptor::Descriptor(
                            RSReference::new(
                                "AdminRoleSuperadmin".into(),
                                GTDefinitionId("module".into(), "AdminRoleSuperadmin".into())
                            )
                            .into()
                        ),
                    },
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "Admin".into(),
                        descriptor: RSEnumVariantDescriptor::Descriptor(
                            RSReference::new(
                                "AdminRoleAdmin".into(),
                                GTDefinitionId("module".into(), "AdminRoleAdmin".into())
                            )
                            .into()
                        ),
                    },
                    RSEnumVariant {
                        doc: None,
                        attributes: vec![],
                        name: "Moderator".into(),
                        descriptor: RSEnumVariantDescriptor::Descriptor(
                            RSReference::new(
                                "AdminRoleModerator".into(),
                                GTDefinitionId("module".into(), "AdminRoleModerator".into())
                            )
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
                    "derive(Default, Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)"
                        .into(),
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
}
