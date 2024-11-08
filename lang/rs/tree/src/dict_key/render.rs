use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::RSDictKey;

impl GTRender for RSDictKey {
    fn render(&self, _indent: &GTIndent) -> String {
        match self {
            RSDictKey::String => "str".into(),
            RSDictKey::Int => "int".into(),
            RSDictKey::Float => "float".into(),
            RSDictKey::Boolean => "bool".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indent::rs_indent;

    #[test]
    fn test_render() {
        assert_eq!(RSDictKey::Boolean.render(&rs_indent()), "bool");
        assert_eq!(RSDictKey::String.render(&rs_indent()), "str");
        assert_eq!(RSDictKey::Int.render(&rs_indent()), "int");
        assert_eq!(RSDictKey::Float.render(&rs_indent()), "float");
    }
}
