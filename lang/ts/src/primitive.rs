use genotype_lang_core::{indent::Indent, node::Node};

pub enum TSPrimitive {
    String,
    Number,
    Boolean,
    Null,
    Undefined,
}

impl Node for TSPrimitive {
    fn render(&self, _indent: &Indent) -> String {
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
        let indent = ts_indent();
        assert_eq!(TSPrimitive::String.render(&indent), "string");
        assert_eq!(TSPrimitive::Number.render(&indent), "number");
        assert_eq!(TSPrimitive::Boolean.render(&indent), "boolean");
        assert_eq!(TSPrimitive::Null.render(&indent), "null");
        assert_eq!(TSPrimitive::Undefined.render(&indent), "undefined");
    }
}
