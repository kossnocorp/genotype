use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for RsPrimitive {
    type RenderState = RsRenderState;

    type RenderContext = RsRenderContext<'a>;

    fn render(
        &self,
        _state: Self::RenderState,
        _context: &mut Self::RenderContext,
    ) -> Result<String> {
        Ok(match self {
            RsPrimitive::Unit => "()",
            RsPrimitive::Boolean => "bool",
            RsPrimitive::String => "String",
            RsPrimitive::Int8 => "i8",
            RsPrimitive::Int16 => "i16",
            RsPrimitive::Int32 => "i32",
            RsPrimitive::Int64 => "i64",
            RsPrimitive::Int128 => "i128",
            RsPrimitive::IntSize => "isize",
            RsPrimitive::IntU8 => "u8",
            RsPrimitive::IntU16 => "u16",
            RsPrimitive::IntU32 => "u32",
            RsPrimitive::IntU64 => "u64",
            RsPrimitive::IntU128 => "u128",
            RsPrimitive::IntUSize => "usize",
            RsPrimitive::Float32 => "f32",
            RsPrimitive::Float64 => "f64",
        }
        .to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_primitive() {
        assert_snapshot!(
            RsPrimitive::Unit
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"()"
        );
        assert_snapshot!(
            RsPrimitive::Boolean
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"bool"
        );
        assert_snapshot!(
            RsPrimitive::String
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"String"
        );
        assert_snapshot!(
            RsPrimitive::Int8
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"i8"
        );
        assert_snapshot!(
            RsPrimitive::Int16
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"i16"
        );
        assert_snapshot!(
            RsPrimitive::Int32
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"i32"
        );
        assert_snapshot!(
            RsPrimitive::Int64
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"i64"
        );
        assert_snapshot!(
            RsPrimitive::Int128
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"i128"
        );
        assert_snapshot!(
            RsPrimitive::IntSize
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"isize"
        );
        assert_snapshot!(
            RsPrimitive::IntU8
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"u8"
        );
        assert_snapshot!(
            RsPrimitive::IntU16
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"u16"
        );
        assert_snapshot!(
            RsPrimitive::IntU32
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"u32"
        );
        assert_snapshot!(
            RsPrimitive::IntU64
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"u64"
        );
        assert_snapshot!(
            RsPrimitive::IntU128
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"u128"
        );
        assert_snapshot!(
            RsPrimitive::IntUSize
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"usize"
        );
        assert_snapshot!(
            RsPrimitive::Float32
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"f32"
        );
        assert_snapshot!(
            RsPrimitive::Float64
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"f64"
        );
    }
}
