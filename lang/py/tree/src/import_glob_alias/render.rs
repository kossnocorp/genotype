use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::PYImportGlobAlias;

impl GTRender for PYImportGlobAlias {
    fn render(&self, _indent: &GTIndent) -> String {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indent::py_indent;

    #[test]
    fn test_render_resolved() {
        assert_eq!(
            PYImportGlobAlias("Name".into()).render(&py_indent()),
            "Name"
        );
    }
}
