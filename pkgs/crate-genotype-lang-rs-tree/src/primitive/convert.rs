use crate::prelude::internal::*;

impl RsConvert<RsPrimitive> for GtPrimitive {
    fn convert(&self, context: &mut RsConvertContext) -> Result<RsPrimitive> {
        Ok(match self.kind {
            GtPrimitiveKind::Boolean => RsPrimitive::Boolean,
            GtPrimitiveKind::String => RsPrimitive::String,
            GtPrimitiveKind::Number => {
                if context.config().needs_ordered_floats() {
                    context.add_import(RsDependencyIdent::OrderedFloat, "OrderedFloat".into());
                }
                RsPrimitive::Float64
            }
            GtPrimitiveKind::Int8 => RsPrimitive::Int8,
            GtPrimitiveKind::Int16 => RsPrimitive::Int16,
            GtPrimitiveKind::Int32 => RsPrimitive::Int32,
            GtPrimitiveKind::Int64 => RsPrimitive::Int64,
            GtPrimitiveKind::Int128 => RsPrimitive::Int128,
            GtPrimitiveKind::IntSize => RsPrimitive::IntSize,
            GtPrimitiveKind::IntU8 => RsPrimitive::IntU8,
            GtPrimitiveKind::IntU16 => RsPrimitive::IntU16,
            GtPrimitiveKind::IntU32 => RsPrimitive::IntU32,
            GtPrimitiveKind::IntU64 => RsPrimitive::IntU64,
            GtPrimitiveKind::IntU128 => RsPrimitive::IntU128,
            GtPrimitiveKind::IntUSize => RsPrimitive::IntUSize,
            GtPrimitiveKind::Float32 => {
                if context.config().needs_ordered_floats() {
                    context.add_import(RsDependencyIdent::OrderedFloat, "OrderedFloat".into());
                }
                RsPrimitive::Float32
            }
            GtPrimitiveKind::Float64 => {
                if context.config().needs_ordered_floats() {
                    context.add_import(RsDependencyIdent::OrderedFloat, "OrderedFloat".into());
                }
                RsPrimitive::Float64
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_test::*;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            Gt::primitive_boolean()
                .convert(&mut RsConvertContext::empty("module".into()))
                .unwrap(),
            @"Boolean"
        );
        assert_ron_snapshot!(
            Gt::primitive_string()
                .convert(&mut RsConvertContext::empty("module".into()))
                .unwrap(),
            @"String"
        );
        assert_ron_snapshot!(
            Gt::primitive_i8()
                .convert(&mut RsConvertContext::empty("module".into()))
                .unwrap(),
            @"Int8"
        );
        assert_ron_snapshot!(
            Gt::primitive_i16()
                .convert(&mut RsConvertContext::empty("module".into()))
                .unwrap(),
            @"Int16"
        );
        assert_ron_snapshot!(
            Gt::primitive_i32()
                .convert(&mut RsConvertContext::empty("module".into()))
                .unwrap(),
            @"Int32"
        );
        assert_ron_snapshot!(
            Gt::primitive_i64()
                .convert(&mut RsConvertContext::empty("module".into()))
                .unwrap(),
            @"Int64"
        );
        assert_ron_snapshot!(
            Gt::primitive_i128()
                .convert(&mut RsConvertContext::empty("module".into()))
                .unwrap(),
            @"Int128"
        );
        assert_ron_snapshot!(
            Gt::primitive_isize()
                .convert(&mut RsConvertContext::empty("module".into()))
                .unwrap(),
            @"IntSize"
        );
        assert_ron_snapshot!(
            Gt::primitive_u8()
                .convert(&mut RsConvertContext::empty("module".into()))
                .unwrap(),
            @"IntU8"
        );
        assert_ron_snapshot!(
            Gt::primitive_u16()
                .convert(&mut RsConvertContext::empty("module".into()))
                .unwrap(),
            @"IntU16"
        );
        assert_ron_snapshot!(
            Gt::primitive_u32()
                .convert(&mut RsConvertContext::empty("module".into()))
                .unwrap(),
            @"IntU32"
        );
        assert_ron_snapshot!(
            Gt::primitive_u64()
                .convert(&mut RsConvertContext::empty("module".into()))
                .unwrap(),
            @"IntU64"
        );
        assert_ron_snapshot!(
            Gt::primitive_u128()
                .convert(&mut RsConvertContext::empty("module".into()))
                .unwrap(),
            @"IntU128"
        );
        assert_ron_snapshot!(
            Gt::primitive_usize()
                .convert(&mut RsConvertContext::empty("module".into()))
                .unwrap(),
            @"IntUSize"
        );
        assert_ron_snapshot!(
            Gt::primitive_f32()
                .convert(&mut RsConvertContext::empty("module".into()))
                .unwrap(),
            @"Float32"
        );
        assert_ron_snapshot!(
            Gt::primitive_f64()
                .convert(&mut RsConvertContext::empty("module".into()))
                .unwrap(),
            @"Float64"
        );
    }

    #[test]
    fn test_convert_float_adds_ordered_float_import_when_needed() {
        let mut context = RsConvertContext::new(
            "module".into(),
            Default::default(),
            RsConfigLang {
                derive: vec!["Debug".into(), "Eq".into()],
            },
            Default::default(),
        );

        assert_ron_snapshot!(
            Gt::primitive_f64().convert(&mut context).unwrap(),
            @"Float64"
        );

        assert_ron_snapshot!(
            context.as_dependencies(),
            @r#"
        [
          (OrderedFloat, RsIdentifier("OrderedFloat")),
        ]
        "#
        );
    }
}
