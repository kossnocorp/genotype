use crate::prelude::internal::*;

impl PYConvert<PYPrimitive> for GTPrimitive {
    fn convert(&self, _resolve: &mut PYConvertContext) -> PYPrimitive {
        match self.kind {
            GTPrimitiveKind::Boolean => PYPrimitive::Boolean,
            GTPrimitiveKind::String => PYPrimitive::String,
            GTPrimitiveKind::Number => PYPrimitive::Float,
            GTPrimitiveKind::Int8 => PYPrimitive::Int,
            GTPrimitiveKind::Int16 => PYPrimitive::Int,
            GTPrimitiveKind::Int32 => PYPrimitive::Int,
            GTPrimitiveKind::Int64 => PYPrimitive::Int,
            GTPrimitiveKind::Int128 => PYPrimitive::Int,
            GTPrimitiveKind::IntSize => PYPrimitive::Int,
            GTPrimitiveKind::IntU8 => PYPrimitive::Int,
            GTPrimitiveKind::IntU16 => PYPrimitive::Int,
            GTPrimitiveKind::IntU32 => PYPrimitive::Int,
            GTPrimitiveKind::IntU64 => PYPrimitive::Int,
            GTPrimitiveKind::IntU128 => PYPrimitive::Int,
            GTPrimitiveKind::IntUSize => PYPrimitive::Int,
            GTPrimitiveKind::Float32 => PYPrimitive::Float,
            GTPrimitiveKind::Float64 => PYPrimitive::Float,
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
            GtFactory::primitive_boolean().convert(&mut PYConvertContext::default()),
            @"Boolean"
        );
        assert_ron_snapshot!(
            GtFactory::primitive_string().convert(&mut PYConvertContext::default()),
            @"String"
        );
        assert_ron_snapshot!(
            GtFactory::primitive_number().convert(&mut PYConvertContext::default()),
            @"Float"
        );
        assert_ron_snapshot!(
            GtFactory::primitive_i8().convert(&mut PYConvertContext::default()),
            @"Int"
        );
        assert_ron_snapshot!(
            GtFactory::primitive_i16().convert(&mut PYConvertContext::default()),
            @"Int"
        );
        assert_ron_snapshot!(
            GtFactory::primitive_i32().convert(&mut PYConvertContext::default()),
            @"Int"
        );
        assert_ron_snapshot!(
            GtFactory::primitive_i64().convert(&mut PYConvertContext::default()),
            @"Int"
        );
        assert_ron_snapshot!(
            GtFactory::primitive_i128().convert(&mut PYConvertContext::default()),
            @"Int"
        );
        assert_ron_snapshot!(
            GtFactory::primitive_isize().convert(&mut PYConvertContext::default()),
            @"Int"
        );
        assert_ron_snapshot!(
            GtFactory::primitive_u8().convert(&mut PYConvertContext::default()),
            @"Int"
        );
        assert_ron_snapshot!(
            GtFactory::primitive_u16().convert(&mut PYConvertContext::default()),
            @"Int"
        );
        assert_ron_snapshot!(
            GtFactory::primitive_u32().convert(&mut PYConvertContext::default()),
            @"Int"
        );
        assert_ron_snapshot!(
            GtFactory::primitive_u64().convert(&mut PYConvertContext::default()),
            @"Int"
        );
        assert_ron_snapshot!(
            GtFactory::primitive_u128().convert(&mut PYConvertContext::default()),
            @"Int"
        );
        assert_ron_snapshot!(
            GtFactory::primitive_usize().convert(&mut PYConvertContext::default()),
            @"Int"
        );
        assert_ron_snapshot!(
            GtFactory::primitive_f64().convert(&mut PYConvertContext::default()),
            @"Float"
        );
        assert_ron_snapshot!(
            GtFactory::primitive_f32().convert(&mut PYConvertContext::default()),
            @"Float"
        );
        assert_ron_snapshot!(
            GtFactory::primitive_f64().convert(&mut PYConvertContext::default()),
            @"Float"
        );
    }
}
