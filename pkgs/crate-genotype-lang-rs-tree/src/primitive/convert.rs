use crate::prelude::internal::*;

impl RSConvert<RSPrimitive> for GTPrimitive {
    fn convert(&self, _resolve: &mut RSConvertContext) -> Result<RSPrimitive> {
        Ok(match self.kind {
            GTPrimitiveKind::Boolean => RSPrimitive::Boolean,
            GTPrimitiveKind::String => RSPrimitive::String,
            GTPrimitiveKind::Number => RSPrimitive::Float64,
            GTPrimitiveKind::Int8 => RSPrimitive::Int8,
            GTPrimitiveKind::Int16 => RSPrimitive::Int16,
            GTPrimitiveKind::Int32 => RSPrimitive::Int32,
            GTPrimitiveKind::Int64 => RSPrimitive::Int64,
            GTPrimitiveKind::Int128 => RSPrimitive::Int128,
            GTPrimitiveKind::IntSize => RSPrimitive::IntSize,
            GTPrimitiveKind::IntU8 => RSPrimitive::IntU8,
            GTPrimitiveKind::IntU16 => RSPrimitive::IntU16,
            GTPrimitiveKind::IntU32 => RSPrimitive::IntU32,
            GTPrimitiveKind::IntU64 => RSPrimitive::IntU64,
            GTPrimitiveKind::IntU128 => RSPrimitive::IntU128,
            GTPrimitiveKind::IntUSize => RSPrimitive::IntUSize,
            GTPrimitiveKind::Float32 => RSPrimitive::Float32,
            GTPrimitiveKind::Float64 => RSPrimitive::Float64,
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
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"Boolean"
        );
        assert_ron_snapshot!(
            Gt::primitive_string()
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"String"
        );
        assert_ron_snapshot!(
            Gt::primitive_i8()
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"Int8"
        );
        assert_ron_snapshot!(
            Gt::primitive_i16()
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"Int16"
        );
        assert_ron_snapshot!(
            Gt::primitive_i32()
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"Int32"
        );
        assert_ron_snapshot!(
            Gt::primitive_i64()
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"Int64"
        );
        assert_ron_snapshot!(
            Gt::primitive_i128()
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"Int128"
        );
        assert_ron_snapshot!(
            Gt::primitive_isize()
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"IntSize"
        );
        assert_ron_snapshot!(
            Gt::primitive_u8()
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"IntU8"
        );
        assert_ron_snapshot!(
            Gt::primitive_u16()
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"IntU16"
        );
        assert_ron_snapshot!(
            Gt::primitive_u32()
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"IntU32"
        );
        assert_ron_snapshot!(
            Gt::primitive_u64()
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"IntU64"
        );
        assert_ron_snapshot!(
            Gt::primitive_u128()
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"IntU128"
        );
        assert_ron_snapshot!(
            Gt::primitive_usize()
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"IntUSize"
        );
        assert_ron_snapshot!(
            Gt::primitive_f32()
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"Float32"
        );
        assert_ron_snapshot!(
            Gt::primitive_f64()
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @"Float64"
        );
    }
}
