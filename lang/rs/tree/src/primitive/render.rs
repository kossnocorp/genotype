use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::RSPrimitive;

impl GTRender for RSPrimitive {
    fn render(&self, _indent: &GTIndent) -> String {
        match self {
            RSPrimitive::Unit => "()",
            RSPrimitive::Boolean => "bool",
            RSPrimitive::String => "String",
            RSPrimitive::Int8 => "i8",
            RSPrimitive::Int16 => "i16",
            RSPrimitive::Int32 => "i32",
            RSPrimitive::Int64 => "i64",
            RSPrimitive::Int128 => "i128",
            RSPrimitive::Int => "isize",
            RSPrimitive::UInt8 => "u8",
            RSPrimitive::UInt16 => "u16",
            RSPrimitive::UInt32 => "u32",
            RSPrimitive::UInt64 => "u64",
            RSPrimitive::UInt128 => "u128",
            RSPrimitive::UInt => "usize",
            RSPrimitive::Float32 => "f32",
            RSPrimitive::Float64 => "f64",
            // [TODO] Figure out how to handle this, likely a runtime crate.
            RSPrimitive::Null => "Null",
        }
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indent::rs_indent;

    #[test]
    fn test_render_primitive() {
        assert_eq!(RSPrimitive::Unit.render(&rs_indent()), "()");
        assert_eq!(RSPrimitive::Boolean.render(&rs_indent()), "bool");
        assert_eq!(RSPrimitive::String.render(&rs_indent()), "String");
        assert_eq!(RSPrimitive::Int8.render(&rs_indent()), "i8");
        assert_eq!(RSPrimitive::Int16.render(&rs_indent()), "i16");
        assert_eq!(RSPrimitive::Int32.render(&rs_indent()), "i32");
        assert_eq!(RSPrimitive::Int64.render(&rs_indent()), "i64");
        assert_eq!(RSPrimitive::Int128.render(&rs_indent()), "i128");
        assert_eq!(RSPrimitive::Int.render(&rs_indent()), "isize");
        assert_eq!(RSPrimitive::UInt8.render(&rs_indent()), "u8");
        assert_eq!(RSPrimitive::UInt16.render(&rs_indent()), "u16");
        assert_eq!(RSPrimitive::UInt32.render(&rs_indent()), "u32");
        assert_eq!(RSPrimitive::UInt64.render(&rs_indent()), "u64");
        assert_eq!(RSPrimitive::UInt128.render(&rs_indent()), "u128");
        assert_eq!(RSPrimitive::UInt.render(&rs_indent()), "usize");
        assert_eq!(RSPrimitive::Float32.render(&rs_indent()), "f32");
        assert_eq!(RSPrimitive::Float64.render(&rs_indent()), "f64");
        assert_eq!(RSPrimitive::Null.render(&rs_indent()), "Null");
    }
}
