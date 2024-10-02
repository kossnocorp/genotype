use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::TSPrimitive;

impl GTRender for TSPrimitive {
    fn render(&self, _indent: &GTIndent) -> String {
        match self {
            TSPrimitive::String => "string",
            TSPrimitive::Number => "number",
            TSPrimitive::Boolean => "boolean",
            TSPrimitive::Null => "null",
            TSPrimitive::Undefined => "undefined",
        }
        .to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indent::ts_indent;

    #[test]
    fn test_render_primitive() {
                assert_eq!(TSPrimitive::String.render(&ts_indent()), "string");
        assert_eq!(TSPrimitive::Number.render(&ts_indent()), "number");
        assert_eq!(TSPrimitive::Boolean.render(&ts_indent()), "boolean");
        assert_eq!(TSPrimitive::Null.render(&ts_indent()), "null");
        assert_eq!(TSPrimitive::Undefined.render(&ts_indent()), "undefined");
    }
}
