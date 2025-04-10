use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for RSPrimitive {
    type RenderContext = RSRenderContext<'a>;

    fn render(&self, _context: &mut Self::RenderContext) -> Result<String> {
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

    #[test]
    fn test_render_primitive() {
        assert_eq!(
            RSPrimitive::Unit.render(&mut Default::default()).unwrap(),
            "()"
        );
        assert_eq!(
            RSPrimitive::Boolean
                .render(&mut Default::default())
                .unwrap(),
            "bool"
        );
        assert_eq!(
            RSPrimitive::String.render(&mut Default::default()).unwrap(),
            "String"
        );
        assert_eq!(
            RSPrimitive::Int8.render(&mut Default::default()).unwrap(),
            "i8"
        );
        assert_eq!(
            RSPrimitive::Int16.render(&mut Default::default()).unwrap(),
            "i16"
        );
        assert_eq!(
            RSPrimitive::Int32.render(&mut Default::default()).unwrap(),
            "i32"
        );
        assert_eq!(
            RSPrimitive::Int64.render(&mut Default::default()).unwrap(),
            "i64"
        );
        assert_eq!(
            RSPrimitive::Int128.render(&mut Default::default()).unwrap(),
            "i128"
        );
        assert_eq!(
            RSPrimitive::IntSize
                .render(&mut Default::default())
                .unwrap(),
            "isize"
        );
        assert_eq!(
            RSPrimitive::IntU8.render(&mut Default::default()).unwrap(),
            "u8"
        );
        assert_eq!(
            RSPrimitive::IntU16.render(&mut Default::default()).unwrap(),
            "u16"
        );
        assert_eq!(
            RSPrimitive::IntU32.render(&mut Default::default()).unwrap(),
            "u32"
        );
        assert_eq!(
            RSPrimitive::IntU64.render(&mut Default::default()).unwrap(),
            "u64"
        );
        assert_eq!(
            RSPrimitive::IntU128
                .render(&mut Default::default())
                .unwrap(),
            "u128"
        );
        assert_eq!(
            RSPrimitive::IntUSize
                .render(&mut Default::default())
                .unwrap(),
            "usize"
        );
        assert_eq!(
            RSPrimitive::Float32
                .render(&mut Default::default())
                .unwrap(),
            "f32"
        );
        assert_eq!(
            RSPrimitive::Float64
                .render(&mut Default::default())
                .unwrap(),
            "f64"
        );
    }
}
