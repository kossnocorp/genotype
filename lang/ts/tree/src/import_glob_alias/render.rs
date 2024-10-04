use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::TSImportGlobAlias;

impl GTRender for TSImportGlobAlias {
    fn render(&self, _indent: &GTIndent) -> String {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indent::ts_indent;

    #[test]
    fn test_render_resolved() {
        assert_eq!(
            TSImportGlobAlias("Name".into()).render(&ts_indent()),
            "Name"
        );
    }
}
