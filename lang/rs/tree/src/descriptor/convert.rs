use crate::prelude::internal::*;

impl RSConvert<RSDescriptor> for GTDescriptor {
    fn convert(&self, context: &mut RSConvertContext) -> Result<RSDescriptor> {
        Ok(match self {
            GTDescriptor::Alias(alias) => context
                .hoist(|context| Ok((alias.convert(context)?, alias.span.clone())))?
                .into(),

            GTDescriptor::Array(array) => array.convert(context)?.into(),

            GTDescriptor::InlineImport(import) => import.convert(context)?.into(),

            GTDescriptor::Literal(literal) => context
                .hoist(|context| Ok((literal.convert(context)?, literal.span)))?
                .into(),

            GTDescriptor::Object(object) => context
                .hoist(|context| Ok((object.convert(context)?, object.span.clone())))?
                .into(),

            GTDescriptor::Primitive(primitive) => primitive.convert(context)?.into(),

            GTDescriptor::Record(record) => record.convert(context)?.into(),

            GTDescriptor::Reference(name) => name.convert(context)?.into(),

            GTDescriptor::Tuple(tuple) => tuple.convert(context)?.into(),

            GTDescriptor::Union(union) => context
                .hoist(|context| Ok((union.convert(context)?, union.span.clone())))?
                .into(),

            GTDescriptor::Any(any) => any.convert(context)?.into(),

            GTDescriptor::Branded(branded) => context
                .hoist(|context| Ok((branded.convert(context)?, branded.span.clone())))?
                .into(),
        })
    }
}

impl RSConvert<RSDescriptor> for GTRecordKey {
    fn convert(&self, _context: &mut RSConvertContext) -> Result<RSDescriptor> {
        Ok(match self {
            GTRecordKey::String(_) => RSPrimitive::String.into(),
            GTRecordKey::Number(_) => RSPrimitive::Float64.into(),
            GTRecordKey::Int8(_) => RSPrimitive::Int8.into(),
            GTRecordKey::Int16(_) => RSPrimitive::Int16.into(),
            GTRecordKey::Int32(_) => RSPrimitive::Int32.into(),
            GTRecordKey::Int64(_) => RSPrimitive::Int64.into(),
            GTRecordKey::Int128(_) => RSPrimitive::Int128.into(),
            GTRecordKey::IntSize(_) => RSPrimitive::IntSize.into(),
            GTRecordKey::IntU8(_) => RSPrimitive::IntU8.into(),
            GTRecordKey::IntU16(_) => RSPrimitive::IntU16.into(),
            GTRecordKey::IntU32(_) => RSPrimitive::IntU32.into(),
            GTRecordKey::IntU64(_) => RSPrimitive::IntU64.into(),
            GTRecordKey::IntU128(_) => RSPrimitive::IntU128.into(),
            GTRecordKey::IntUSize(_) => RSPrimitive::IntUSize.into(),
            GTRecordKey::Float32(_) => RSPrimitive::Float32.into(),
            GTRecordKey::Float64(_) => RSPrimitive::Float64.into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;
    use genotype_test::*;

    #[test]
    fn test_convert_descriptor_alias() {
        let mut context = RSConvertContext::empty("module".into());
        assert_ron_snapshot!(
            GTDescriptor::Alias(Box::new(GTAlias {
                id: GTDefinitionId("module".into(), "Name".into()),
                span: (0, 1).into(),
                doc: None,
                attributes: vec![],
                name: GTIdentifier::new((0, 0).into(), "Name".into()),
                descriptor: Gt::primitive_boolean().into(),
            }))
            .convert(&mut context)
            .unwrap(),
            @r#"
        Reference(RSReference(
          id: GTReferenceId(GTModuleId("module"), GTSpan(0, 1)),
          identifier: RSIdentifier("Name"),
          definition_id: GTDefinitionId(GTModuleId("module"), "Name"),
        ))
        "#
        );
        let hoisted = context.drain_hoisted();
        assert_ron_snapshot!(
            hoisted,
            @r#"
        [
          Alias(RSAlias(
            id: GTDefinitionId(GTModuleId("module"), "Name"),
            doc: None,
            name: RSIdentifier("Name"),
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
            .convert(&mut RSConvertContext::empty("module".into()))
            .unwrap(),
            @"
        Vec(RSVec(
          descriptor: Primitive(Boolean),
        ))
        "
        );
    }

    #[test]
    fn test_convert_descriptor_inline_import() {
        assert_ron_snapshot!(
            convert_node(
                Gt::descriptor(Gt::inline_import("./path/to/module", "Name"))
            ),
            @r#"
        InlineUse(RSInlineUse(
          path: RSPath(GTModuleId("path/to/module"), "super::path::to::module"),
          name: RSIdentifier("Name"),
        ))
        "#
        );
    }

    #[test]
    fn test_convert_descriptor_object() {
        let mut context = RSConvertContext::empty("module".into());
        assert_ron_snapshot!(
            GTDescriptor::Object(GTObject {
                span: (0, 1).into(),
                doc: None,
                attributes: vec![],
                name: GTObjectName::Named(GTIdentifier::new((0, 0).into(), "Person".into())),
                extensions: vec![],
                properties: vec![
                    GTProperty {
                        span: (0, 0).into(),
                        doc: None,
                        attributes: vec![],
                        name: GTKey::new((0, 0).into(), "name".into()),
                        descriptor: Gt::primitive_string().into(),
                        required: true,
                    },
                    GTProperty {
                        span: (0, 0).into(),
                        doc: None,
                        attributes: vec![],
                        name: GTKey::new((0, 0).into(), "age".into()),
                        descriptor: Gt::primitive_i32().into(),
                        required: false,
                    }
                ],
            })
            .convert(&mut context)
            .unwrap(),
            @r#"
        Reference(RSReference(
          id: GTReferenceId(GTModuleId("module"), GTSpan(0, 1)),
          identifier: RSIdentifier("Person"),
          definition_id: GTDefinitionId(GTModuleId("module"), "Person"),
        ))
        "#
        );
        let hoisted = context.drain_hoisted();
        assert_ron_snapshot!(
            hoisted,
            @r#"
        [
          Struct(RSStruct(
            id: GTDefinitionId(GTModuleId("module"), "Person"),
            doc: None,
            attributes: [
              RSAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
            ],
            name: RSIdentifier("Person"),
            fields: Resolved([
              RSField(
                doc: None,
                attributes: [],
                name: RSFieldName("name"),
                descriptor: Primitive(String),
              ),
              RSField(
                doc: None,
                attributes: [
                  RSAttribute("serde(default, skip_serializing_if = \"Option::is_none\")"),
                ],
                name: RSFieldName("age"),
                descriptor: Option(RSOption(
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
        assert_ron_snapshot!(
            convert_node(Gt::descriptor(Gt::reference("Name"))),
            @r#"
        Reference(RSReference(
          id: GTReferenceId(GTModuleId("module"), GTSpan(0, 0)),
          identifier: RSIdentifier("Name"),
          definition_id: GTDefinitionId(GTModuleId("module"), "Name"),
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
        Tuple(RSTuple(
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
        let mut context = Gtrs::convert_context_with_parent("Union");
        assert_ron_snapshot!(
            convert_node_with(Gt::descriptor(Gt::union(vec![
                Gt::primitive_boolean().into(),
                Gt::primitive_string().into(),
            ])), &mut context),
            @r#"
        Reference(RSReference(
          id: GTReferenceId(GTModuleId("module"), GTSpan(0, 0)),
          identifier: RSIdentifier("Union"),
          definition_id: GTDefinitionId(GTModuleId("module"), "Union"),
        ))
        "#
        );
        let hoisted = context.drain_hoisted();
        assert_ron_snapshot!(
            hoisted,
            @r#"
        [
          Enum(RSEnum(
            id: GTDefinitionId(GTModuleId("module"), "Union"),
            doc: None,
            attributes: [
              RSAttribute("derive(Debug, Clone, PartialEq, Serialize, Deserialize)"),
              RSAttribute("serde(untagged)"),
            ],
            name: RSIdentifier("Union"),
            variants: [
              RSEnumVariant(
                doc: None,
                attributes: [],
                name: RSIdentifier("Boolean"),
                descriptor: Some(Descriptor(Primitive(Boolean))),
              ),
              RSEnumVariant(
                doc: None,
                attributes: [],
                name: RSIdentifier("String"),
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
}
