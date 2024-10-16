use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::PYPrimitive;

impl GTRender for PYPrimitive {
    fn render(&self, _indent: &GTIndent) -> String {
        match self {
            PYPrimitive::Boolean => "bool",
            PYPrimitive::String => "str",
            PYPrimitive::Int => "int",
            PYPrimitive::Float => "float",
            PYPrimitive::None => "None",
        }
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indent::py_indent;

    #[test]
    fn test_render_primitive() {
        assert_eq!(PYPrimitive::Boolean.render(&py_indent()), "bool");
        assert_eq!(PYPrimitive::String.render(&py_indent()), "str");
        assert_eq!(PYPrimitive::Int.render(&py_indent()), "int");
        assert_eq!(PYPrimitive::Float.render(&py_indent()), "float");
        assert_eq!(PYPrimitive::None.render(&py_indent()), "None");
    }
}
