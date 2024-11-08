use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::RSPrimitive;

impl GTRender for RSPrimitive {
    fn render(&self, _indent: &GTIndent) -> String {
        match self {
            RSPrimitive::Boolean => "bool",
            RSPrimitive::String => "str",
            RSPrimitive::Int => "int",
            RSPrimitive::Float => "float",
            RSPrimitive::None => "None",
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
        assert_eq!(RSPrimitive::Boolean.render(&rs_indent()), "bool");
        assert_eq!(RSPrimitive::String.render(&rs_indent()), "str");
        assert_eq!(RSPrimitive::Int.render(&rs_indent()), "int");
        assert_eq!(RSPrimitive::Float.render(&rs_indent()), "float");
        assert_eq!(RSPrimitive::None.render(&rs_indent()), "None");
    }
}
