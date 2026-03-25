use crate::prelude::internal::*;

impl PyConvert<PyPrimitive> for GtPrimitive {
    fn convert(&self, _resolve: &mut PyConvertContext) -> PyPrimitive {
        match self.kind {
            GtPrimitiveKind::Boolean => PyPrimitive::Boolean,
            GtPrimitiveKind::String => PyPrimitive::String,
            GtPrimitiveKind::Number => PyPrimitive::Float,
            GtPrimitiveKind::Int8 => PyPrimitive::Int,
            GtPrimitiveKind::Int16 => PyPrimitive::Int,
            GtPrimitiveKind::Int32 => PyPrimitive::Int,
            GtPrimitiveKind::Int64 => PyPrimitive::Int,
            GtPrimitiveKind::Int128 => PyPrimitive::Int,
            GtPrimitiveKind::IntSize => PyPrimitive::Int,
            GtPrimitiveKind::IntU8 => PyPrimitive::Int,
            GtPrimitiveKind::IntU16 => PyPrimitive::Int,
            GtPrimitiveKind::IntU32 => PyPrimitive::Int,
            GtPrimitiveKind::IntU64 => PyPrimitive::Int,
            GtPrimitiveKind::IntU128 => PyPrimitive::Int,
            GtPrimitiveKind::IntUSize => PyPrimitive::Int,
            GtPrimitiveKind::Float32 => PyPrimitive::Float,
            GtPrimitiveKind::Float64 => PyPrimitive::Float,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_test::*;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            Gt::primitive_boolean().convert(&mut PyConvertContext::default()),
            @"Boolean"
        );
        assert_ron_snapshot!(
            Gt::primitive_string().convert(&mut PyConvertContext::default()),
            @"String"
        );
        assert_ron_snapshot!(
            Gt::primitive_number().convert(&mut PyConvertContext::default()),
            @"Float"
        );
        assert_ron_snapshot!(
            Gt::primitive_i8().convert(&mut PyConvertContext::default()),
            @"Int"
        );
        assert_ron_snapshot!(
            Gt::primitive_i16().convert(&mut PyConvertContext::default()),
            @"Int"
        );
        assert_ron_snapshot!(
            Gt::primitive_i32().convert(&mut PyConvertContext::default()),
            @"Int"
        );
        assert_ron_snapshot!(
            Gt::primitive_i64().convert(&mut PyConvertContext::default()),
            @"Int"
        );
        assert_ron_snapshot!(
            Gt::primitive_i128().convert(&mut PyConvertContext::default()),
            @"Int"
        );
        assert_ron_snapshot!(
            Gt::primitive_isize().convert(&mut PyConvertContext::default()),
            @"Int"
        );
        assert_ron_snapshot!(
            Gt::primitive_u8().convert(&mut PyConvertContext::default()),
            @"Int"
        );
        assert_ron_snapshot!(
            Gt::primitive_u16().convert(&mut PyConvertContext::default()),
            @"Int"
        );
        assert_ron_snapshot!(
            Gt::primitive_u32().convert(&mut PyConvertContext::default()),
            @"Int"
        );
        assert_ron_snapshot!(
            Gt::primitive_u64().convert(&mut PyConvertContext::default()),
            @"Int"
        );
        assert_ron_snapshot!(
            Gt::primitive_u128().convert(&mut PyConvertContext::default()),
            @"Int"
        );
        assert_ron_snapshot!(
            Gt::primitive_usize().convert(&mut PyConvertContext::default()),
            @"Int"
        );
        assert_ron_snapshot!(
            Gt::primitive_f64().convert(&mut PyConvertContext::default()),
            @"Float"
        );
        assert_ron_snapshot!(
            Gt::primitive_f32().convert(&mut PyConvertContext::default()),
            @"Float"
        );
        assert_ron_snapshot!(
            Gt::primitive_f64().convert(&mut PyConvertContext::default()),
            @"Float"
        );
    }
}
