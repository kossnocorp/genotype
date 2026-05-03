use crate::prelude::internal::*;

impl RsConvert<RsDescriptor> for GtDescriptor {
    fn convert(&self, context: &mut RsConvertContext) -> Result<RsDescriptor> {
        Ok(match self {
            GtDescriptor::Alias(alias) => context
                .hoist(|context| Ok((alias.convert(context)?, alias.span)))?
                .into(),

            GtDescriptor::Array(array) => array.convert(context)?.into(),

            GtDescriptor::InlineImport(import) => import.convert(context)?.into(),

            GtDescriptor::Literal(literal) => context
                .hoist(|context| Ok((literal.convert(context)?, literal.span)))?
                .into(),

            GtDescriptor::Object(object) => context
                .hoist(|context| Ok((object.convert(context)?, object.span)))?
                .into(),

            GtDescriptor::Primitive(primitive) => primitive.convert(context)?.into(),

            GtDescriptor::Record(record) => record.convert(context)?.into(),

            GtDescriptor::Reference(name) => name.convert(context)?.into(),

            GtDescriptor::Tuple(tuple) => tuple.convert(context)?.into(),

            GtDescriptor::Union(union) => context
                .hoist(|context| Ok((union.convert(context)?, union.span)))?
                .into(),

            GtDescriptor::Any(any) => any.convert(context)?.into(),

            GtDescriptor::Branded(branded) => context
                .hoist(|context| Ok((branded.convert(context)?, branded.span)))?
                .into(),
        })
    }
}

impl RsConvert<RsDescriptor> for GtRecordKey {
    fn convert(&self, context: &mut RsConvertContext) -> Result<RsDescriptor> {
        Ok(match self {
            GtRecordKey::String(_) => RsPrimitive::String.into(),
            GtRecordKey::Number(_) => {
                if context.config().needs_ordered_floats() {
                    context.push_import(RsUse::new(
                        RsDependencyIdent::OrderedFloat,
                        "OrderedFloat".into(),
                    ));
                }
                RsPrimitive::Float64.into()
            }
            GtRecordKey::Int8(_) => RsPrimitive::Int8.into(),
            GtRecordKey::Int16(_) => RsPrimitive::Int16.into(),
            GtRecordKey::Int32(_) => RsPrimitive::Int32.into(),
            GtRecordKey::Int64(_) => RsPrimitive::Int64.into(),
            GtRecordKey::Int128(_) => RsPrimitive::Int128.into(),
            GtRecordKey::IntSize(_) => RsPrimitive::IntSize.into(),
            GtRecordKey::IntU8(_) => RsPrimitive::IntU8.into(),
            GtRecordKey::IntU16(_) => RsPrimitive::IntU16.into(),
            GtRecordKey::IntU32(_) => RsPrimitive::IntU32.into(),
            GtRecordKey::IntU64(_) => RsPrimitive::IntU64.into(),
            GtRecordKey::IntU128(_) => RsPrimitive::IntU128.into(),
            GtRecordKey::IntUSize(_) => RsPrimitive::IntUSize.into(),
            GtRecordKey::Float32(_) => {
                if context.config().needs_ordered_floats() {
                    context.push_import(RsUse::new(
                        RsDependencyIdent::OrderedFloat,
                        "OrderedFloat".into(),
                    ));
                }
                RsPrimitive::Float32.into()
            }
            GtRecordKey::Float64(_) => {
                if context.config().needs_ordered_floats() {
                    context.push_import(RsUse::new(
                        RsDependencyIdent::OrderedFloat,
                        "OrderedFloat".into(),
                    ));
                }
                RsPrimitive::Float64.into()
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convert_descriptor_alias() {
        let mut context = RsConvertContext::empty("module".into());
        assert_ron_snapshot!(
            GtDescriptor::Alias(Box::new(GtAlias {
                id: GtDefinitionId("module".into(), "Name".into()),
                span: (0, 1).into(),
                doc: None,
                attributes: vec![],
                name: GtIdentifier::new((0, 0).into(), "Name".into()),
                generics: vec![],
                descriptor: Gt::primitive_boolean().into(),
            }))
            .convert(&mut context)
            .unwrap(),
            @r#"
        Reference(RsReference(
          id: GtReferenceId(GtModuleId("module"), GtSpan(0, 1)),
          identifier: RsIdentifier("Name"),
          definition_id: GtDefinitionId(GtModuleId("module"), "Name"),
        ))
        "#
        );
        let hoisted = context.drain_hoisted();
        assert_ron_snapshot!(
            hoisted,
            @r#"
        [
          Alias(RsAlias(
            id: GtDefinitionId(GtModuleId("module"), "Name"),
            doc: None,
            name: RsIdentifier("Name"),
            descriptor: Primitive(Boolean),
          )),
        ]
        "#
        );
    }

    #[test]
    fn test_convert_descriptor_array() {
        assert_ron_snapshot!(
            Gt::descriptor(Gt::array(Gt::primitive_boolean()))
            .convert(&mut RsConvertContext::empty("module".into()))
            .unwrap(),
            @"
        Vec(RsVec(
          descriptor: Primitive(Boolean),
        ))
        "
        );
    }

    #[test]
    fn test_convert_descriptor_inline_import() {
        let mut context = Rst::convert_context_with(
            vec![(Gt::path_module_id((0, 0)), "module/path".into())],
            vec![],
        );
        assert_ron_snapshot!(
            convert_node_with(
                Gt::descriptor(Gt::inline_import_anon("./path/to/module", "Name")),
                &mut context
            ),
            @r#"
        InlineUse(RsInlineUse(
          path: RsPath(GtModuleId("module/path"), "super::path::to::module"),
          name: RsIdentifier("Name"),
        ))
        "#
        );
    }

    #[test]
    fn test_convert_descriptor_object() {
        let mut context = RsConvertContext::empty("module".into());
        assert_ron_snapshot!(
            GtDescriptor::Object(GtObject {
                span: (0, 1).into(),
                doc: None,
                attributes: vec![],
                name: GtObjectName::Named(GtIdentifier::new((0, 0).into(), "Person".into())),
                extensions: vec![],
                properties: vec![
                    GtProperty {
                        span: (0, 0).into(),
                        doc: None,
                        attributes: vec![],
                        name: GtKey::new((0, 0).into(), "name".into()),
                        descriptor: Gt::primitive_string().into(),
                        required: true,
                    },
                    GtProperty {
                        span: (0, 0).into(),
                        doc: None,
                        attributes: vec![],
                        name: GtKey::new((0, 0).into(), "age".into()),
                        descriptor: Gt::primitive_i32().into(),
                        required: false,
                    }
                ],
            })
            .convert(&mut context)
            .unwrap(),
            @r#"
        Reference(RsReference(
          id: GtReferenceId(GtModuleId("module"), GtSpan(0, 1)),
          identifier: RsIdentifier("Person"),
          definition_id: GtDefinitionId(GtModuleId("module"), "Person"),
        ))
        "#
        );
        let hoisted = context.drain_hoisted();
        assert_ron_snapshot!(
            hoisted,
            @r#"
        [
          Struct(RsStruct(
            id: GtDefinitionId(GtModuleId("module"), "Person"),
            doc: None,
            attributes: [
              RsAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
            ],
            name: RsIdentifier("Person"),
            fields: Resolved([
              RsField(
                doc: None,
                attributes: [],
                name: RsFieldName("name"),
                descriptor: Primitive(String),
              ),
              RsField(
                doc: None,
                attributes: [
                  RsAttribute("serde(default, skip_serializing_if = \"Option::is_none\")"),
                ],
                name: RsFieldName("age"),
                descriptor: Option(RsOption(
                  descriptor: Primitive(Int32),
                )),
              ),
            ]),
          )),
        ]
        "#
        );
    }

    #[test]
    fn test_convert_descriptor_primitive() {
        assert_ron_snapshot!(
            convert_node(Gt::descriptor(Gt::primitive_boolean())),
            @"Primitive(Boolean)"
        );
    }

    #[test]
    fn test_convert_descriptor_reference() {
        let mut context = Rst::convert_context_with(
            vec![],
            vec![(Gt::reference_id((0, 0)), Gt::definition_id("Name"))],
        );
        assert_ron_snapshot!(
            convert_node_with(
                Gt::descriptor(Gt::reference_anon("Name")),
                &mut context
            ),
            @r#"
        Reference(RsReference(
          id: GtReferenceId(GtModuleId("module"), GtSpan(0, 0)),
          identifier: RsIdentifier("Name"),
          definition_id: GtDefinitionId(GtModuleId("module"), "Name"),
        ))
        "#
        );
    }

    #[test]
    fn test_convert_descriptor_tuple() {
        assert_ron_snapshot!(
            convert_node(Gt::descriptor(Gt::tuple(vec![
                Gt::primitive_boolean().into(),
                Gt::primitive_string().into(),
            ]))),
            @"
        Tuple(RsTuple(
          descriptors: [
            Primitive(Boolean),
            Primitive(String),
          ],
        ))
        "
        );
    }

    #[test]
    fn test_convert_descriptor_union() {
        let mut context = Rst::convert_context();
        context.enter_parent(Rst::context_parent("Union"));
        assert_ron_snapshot!(
            convert_node_with(Gt::descriptor(Gt::union(vec![
                Gt::primitive_boolean().into(),
                Gt::primitive_string().into(),
            ])), &mut context),
            @r#"
        Reference(RsReference(
          id: GtReferenceId(GtModuleId("module"), GtSpan(0, 0)),
          identifier: RsIdentifier("Union"),
          definition_id: GtDefinitionId(GtModuleId("module"), "Union"),
        ))
        "#
        );
        let hoisted = context.drain_hoisted();
        assert_ron_snapshot!(
            hoisted,
            @r#"
        [
          Enum(RsEnum(
            id: GtDefinitionId(GtModuleId("module"), "Union"),
            doc: None,
            attributes: [
              RsAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
              RsAttribute("serde(untagged)"),
            ],
            name: RsIdentifier("Union"),
            variants: [
              RsEnumVariant(
                doc: None,
                attributes: [],
                name: RsIdentifier("Boolean"),
                descriptor: Some(Descriptor(Primitive(Boolean))),
              ),
              RsEnumVariant(
                doc: None,
                attributes: [],
                name: RsIdentifier("String"),
                descriptor: Some(Descriptor(Primitive(String))),
              ),
            ],
          )),
        ]
        "#
        );
    }

    // Record key

    #[test]
    fn test_convert_record_key() {
        assert_ron_snapshot!(
            convert_node(Gt::record_key_string()),
            @"Primitive(String)"
        );
        assert_ron_snapshot!(
            convert_node(Gt::record_key_i8()),
            @"Primitive(Int8)"
        );
        assert_ron_snapshot!(
            convert_node(Gt::record_key_i16()),
            @"Primitive(Int16)"
        );
        assert_ron_snapshot!(
            convert_node(Gt::record_key_i32()),
            @"Primitive(Int32)"
        );
        assert_ron_snapshot!(
            convert_node(Gt::record_key_i64()),
            @"Primitive(Int64)"
        );
        assert_ron_snapshot!(
            convert_node(Gt::record_key_i128()),
            @"Primitive(Int128)"
        );
        assert_ron_snapshot!(
            convert_node(Gt::record_key_isize()),
            @"Primitive(IntSize)"
        );
        assert_ron_snapshot!(
            convert_node(Gt::record_key_u8()),
            @"Primitive(IntU8)"
        );
        assert_ron_snapshot!(
            convert_node(Gt::record_key_u16()),
            @"Primitive(IntU16)"
        );
        assert_ron_snapshot!(
            convert_node(Gt::record_key_u32()),
            @"Primitive(IntU32)"
        );
        assert_ron_snapshot!(
            convert_node(Gt::record_key_u64()),
            @"Primitive(IntU64)"
        );
        assert_ron_snapshot!(
            convert_node(Gt::record_key_u128()),
            @"Primitive(IntU128)"
        );
        assert_ron_snapshot!(
            convert_node(Gt::record_key_usize()),
            @"Primitive(IntUSize)"
        );
        assert_ron_snapshot!(
            convert_node(Gt::record_key_f32()),
            @"Primitive(Float32)"
        );
        assert_ron_snapshot!(
            convert_node(Gt::record_key_f64()),
            @"Primitive(Float64)"
        );
    }

    #[test]
    fn test_convert_record_key_float_import_when_needed() {
        let mut context = RsConvertContext::new(
            "module".into(),
            Default::default(),
            RsConfigLang {
                derive: vec!["Debug".into(), "Eq".into()],
            },
            Default::default(),
        );

        assert_ron_snapshot!(
            convert_node_with(Gt::record_key_f64(), &mut context),
            @"Primitive(Float64)"
        );

        assert_ron_snapshot!(
            context.imports(),
            @r#"
        [
          RsUse(
            dependency: OrderedFloat,
            reference: Named([
              Name(RsIdentifier("OrderedFloat")),
            ]),
          ),
        ]
        "#
        );
    }
}
