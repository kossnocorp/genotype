use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::PYDictKey;

impl GTRender for PYDictKey {
    fn render(&self, _indent: &GTIndent) -> String {
        match self {
            PYDictKey::String => "str".into(),
            PYDictKey::Int => "int".into(),
            PYDictKey::Float => "float".into(),
            PYDictKey::Boolean => "bool".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indent::py_indent;

    #[test]
    fn test_render() {
        assert_eq!(PYDictKey::Boolean.render(&py_indent()), "bool");
        assert_eq!(PYDictKey::String.render(&py_indent()), "str");
        assert_eq!(PYDictKey::Int.render(&py_indent()), "int");
        assert_eq!(PYDictKey::Float.render(&py_indent()), "float");
    }
}
