use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::TSRecordKey;

impl GTRender for TSRecordKey {
    fn render(&self, _indent: &GTIndent) -> String {
        match self {
            TSRecordKey::String => "string".into(),
            TSRecordKey::Number => "number".into(),
            TSRecordKey::Boolean => "boolean".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indent::ts_indent;

    #[test]
    fn test_render() {
        assert_eq!(TSRecordKey::String.render(&ts_indent()), "string");
        assert_eq!(TSRecordKey::Number.render(&ts_indent()), "number");
        assert_eq!(TSRecordKey::Boolean.render(&ts_indent()), "boolean");
    }
}
