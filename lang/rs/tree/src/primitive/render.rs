use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;
use miette::Result;

use crate::RSRender;

use super::RSPrimitive;

impl RSRender for RSPrimitive {
    fn render(&self, _indent: &GTIndent, _config: &RSLangConfig) -> Result<String> {
        Ok(match self {
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
        }
        .to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indent::rs_indent;

    #[test]
    fn test_render_primitive() {
        assert_eq!(
            RSPrimitive::Unit
                .render(&rs_indent(), &Default::default())
                .unwrap(),
            "()"
        );
        assert_eq!(
            RSPrimitive::Boolean
                .render(&rs_indent(), &Default::default())
                .unwrap(),
            "bool"
        );
        assert_eq!(
            RSPrimitive::String
                .render(&rs_indent(), &Default::default())
                .unwrap(),
            "String"
        );
        assert_eq!(
            RSPrimitive::Int8
                .render(&rs_indent(), &Default::default())
                .unwrap(),
            "i8"
        );
        assert_eq!(
            RSPrimitive::Int16
                .render(&rs_indent(), &Default::default())
                .unwrap(),
            "i16"
        );
        assert_eq!(
            RSPrimitive::Int32
                .render(&rs_indent(), &Default::default())
                .unwrap(),
            "i32"
        );
        assert_eq!(
            RSPrimitive::Int64
                .render(&rs_indent(), &Default::default())
                .unwrap(),
            "i64"
        );
        assert_eq!(
            RSPrimitive::Int128
                .render(&rs_indent(), &Default::default())
                .unwrap(),
            "i128"
        );
        assert_eq!(
            RSPrimitive::Int
                .render(&rs_indent(), &Default::default())
                .unwrap(),
            "isize"
        );
        assert_eq!(
            RSPrimitive::UInt8
                .render(&rs_indent(), &Default::default())
                .unwrap(),
            "u8"
        );
        assert_eq!(
            RSPrimitive::UInt16
                .render(&rs_indent(), &Default::default())
                .unwrap(),
            "u16"
        );
        assert_eq!(
            RSPrimitive::UInt32
                .render(&rs_indent(), &Default::default())
                .unwrap(),
            "u32"
        );
        assert_eq!(
            RSPrimitive::UInt64
                .render(&rs_indent(), &Default::default())
                .unwrap(),
            "u64"
        );
        assert_eq!(
            RSPrimitive::UInt128
                .render(&rs_indent(), &Default::default())
                .unwrap(),
            "u128"
        );
        assert_eq!(
            RSPrimitive::UInt
                .render(&rs_indent(), &Default::default())
                .unwrap(),
            "usize"
        );
        assert_eq!(
            RSPrimitive::Float32
                .render(&rs_indent(), &Default::default())
                .unwrap(),
            "f32"
        );
        assert_eq!(
            RSPrimitive::Float64
                .render(&rs_indent(), &Default::default())
                .unwrap(),
            "f64"
        );
    }
}
