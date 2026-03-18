use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for RSPrimitive {
    type RenderState = RSRenderState;

    type RenderContext = RSRenderContext<'a>;

    fn render(
        &self,
        _state: Self::RenderState,
        _context: &mut Self::RenderContext,
    ) -> Result<String> {
        Ok(match self {
            RSPrimitive::Unit => "()",
            RSPrimitive::Boolean => "bool",
            RSPrimitive::String => "String",
            RSPrimitive::Int8 => "i8",
            RSPrimitive::Int16 => "i16",
            RSPrimitive::Int32 => "i32",
            RSPrimitive::Int64 => "i64",
            RSPrimitive::Int128 => "i128",
            RSPrimitive::IntSize => "isize",
            RSPrimitive::IntU8 => "u8",
            RSPrimitive::IntU16 => "u16",
            RSPrimitive::IntU32 => "u32",
            RSPrimitive::IntU64 => "u64",
            RSPrimitive::IntU128 => "u128",
            RSPrimitive::IntUSize => "usize",
            RSPrimitive::Float32 => "f32",
            RSPrimitive::Float64 => "f64",
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
            RSPrimitive::Unit
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"()"
        );
        assert_snapshot!(
            RSPrimitive::Boolean
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"bool"
        );
        assert_snapshot!(
            RSPrimitive::String
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"String"
        );
        assert_snapshot!(
            RSPrimitive::Int8
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"i8"
        );
        assert_snapshot!(
            RSPrimitive::Int16
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"i16"
        );
        assert_snapshot!(
            RSPrimitive::Int32
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"i32"
        );
        assert_snapshot!(
            RSPrimitive::Int64
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"i64"
        );
        assert_snapshot!(
            RSPrimitive::Int128
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"i128"
        );
        assert_snapshot!(
            RSPrimitive::IntSize
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"isize"
        );
        assert_snapshot!(
            RSPrimitive::IntU8
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"u8"
        );
        assert_snapshot!(
            RSPrimitive::IntU16
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"u16"
        );
        assert_snapshot!(
            RSPrimitive::IntU32
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"u32"
        );
        assert_snapshot!(
            RSPrimitive::IntU64
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"u64"
        );
        assert_snapshot!(
            RSPrimitive::IntU128
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"u128"
        );
        assert_snapshot!(
            RSPrimitive::IntUSize
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"usize"
        );
        assert_snapshot!(
            RSPrimitive::Float32
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"f32"
        );
        assert_snapshot!(
            RSPrimitive::Float64
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"f64"
        );
    }
}
