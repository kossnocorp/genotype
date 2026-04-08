use crate::prelude::internal::*;

impl TsConvert<TsPrimitive> for GtPrimitive {
    fn convert(&self, _context: &mut TsConvertContext) -> TsPrimitive {
        match self.kind {
            GtPrimitiveKind::Boolean => TsPrimitive::Boolean,
            GtPrimitiveKind::String => TsPrimitive::String,
            GtPrimitiveKind::Number => TsPrimitive::Number,
            GtPrimitiveKind::Int8 => TsPrimitive::Number,
            GtPrimitiveKind::Int16 => TsPrimitive::Number,
            GtPrimitiveKind::Int32 => TsPrimitive::Number,
            GtPrimitiveKind::Int64 => TsPrimitive::Number,
            GtPrimitiveKind::Int128 => TsPrimitive::BigInt,
            GtPrimitiveKind::IntSize => TsPrimitive::Number,
            GtPrimitiveKind::IntU8 => TsPrimitive::Number,
            GtPrimitiveKind::IntU16 => TsPrimitive::Number,
            GtPrimitiveKind::IntU32 => TsPrimitive::Number,
            GtPrimitiveKind::IntU64 => TsPrimitive::Number,
            GtPrimitiveKind::IntU128 => TsPrimitive::BigInt,
            GtPrimitiveKind::IntUSize => TsPrimitive::Number,
            GtPrimitiveKind::Float32 => TsPrimitive::Number,
            GtPrimitiveKind::Float64 => TsPrimitive::Number,
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
            convert_node(Gt::primitive_boolean()),
            @"Boolean"
        );
        assert_ron_snapshot!(
            convert_node(Gt::primitive_string()),
            @"String"
        );
        assert_ron_snapshot!(
            convert_node(Gt::primitive_i8()),
            @"Number"
        );
        assert_ron_snapshot!(
            convert_node(Gt::primitive_i16()),
            @"Number"
        );
        assert_ron_snapshot!(
            convert_node(Gt::primitive_i32()),
            @"Number"
        );
        assert_ron_snapshot!(
            convert_node(Gt::primitive_i64()),
            @"Number"
        );
        assert_ron_snapshot!(
            convert_node(Gt::primitive_i128()),
            @"BigInt"
        );
        assert_ron_snapshot!(
            convert_node(Gt::primitive_isize()),
            @"Number"
        );
        assert_ron_snapshot!(
            convert_node(Gt::primitive_u8()),
            @"Number"
        );
        assert_ron_snapshot!(
            convert_node(Gt::primitive_u16()),
            @"Number"
        );
        assert_ron_snapshot!(
            convert_node(Gt::primitive_u32()),
            @"Number"
        );
        assert_ron_snapshot!(
            convert_node(Gt::primitive_u64()),
            @"Number"
        );
        assert_ron_snapshot!(
            convert_node(Gt::primitive_u128()),
            @"BigInt"
        );
        assert_ron_snapshot!(
            convert_node(Gt::primitive_usize()),
            @"Number"
        );
        assert_ron_snapshot!(
            convert_node(Gt::primitive_f32()),
            @"Number"
        );
        assert_ron_snapshot!(
            convert_node(Gt::primitive_f64()),
            @"Number"
        );
    }
}
