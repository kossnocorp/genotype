use crate::prelude::internal::*;

impl TSConvert<TSPrimitive> for GTPrimitive {
    fn convert(&self, _context: &mut TSConvertContext) -> TSPrimitive {
        match self.kind {
            GTPrimitiveKind::Boolean => TSPrimitive::Boolean,
            GTPrimitiveKind::String => TSPrimitive::String,
            GTPrimitiveKind::Number => TSPrimitive::Number,
            GTPrimitiveKind::Int8 => TSPrimitive::Number,
            GTPrimitiveKind::Int16 => TSPrimitive::Number,
            GTPrimitiveKind::Int32 => TSPrimitive::Number,
            GTPrimitiveKind::Int64 => TSPrimitive::Number,
            GTPrimitiveKind::Int128 => TSPrimitive::BigInt,
            GTPrimitiveKind::IntSize => TSPrimitive::Number,
            GTPrimitiveKind::IntU8 => TSPrimitive::Number,
            GTPrimitiveKind::IntU16 => TSPrimitive::Number,
            GTPrimitiveKind::IntU32 => TSPrimitive::Number,
            GTPrimitiveKind::IntU64 => TSPrimitive::Number,
            GTPrimitiveKind::IntU128 => TSPrimitive::BigInt,
            GTPrimitiveKind::IntUSize => TSPrimitive::Number,
            GTPrimitiveKind::Float32 => TSPrimitive::Number,
            GTPrimitiveKind::Float64 => TSPrimitive::Number,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;
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
