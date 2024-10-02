use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::TSImportGlobAlias;

impl GTRender for TSImportGlobAlias {
    fn render(&self, _indent: &GTIndent) -> String {
        match self {
            TSImportGlobAlias::Resolved(name) => name.clone(),

            TSImportGlobAlias::Unresolved => panic!("Tried to render unresolved glob import alias"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indent::ts_indent;

    #[test]
    fn test_render_resolved() {
        assert_eq!(
            TSImportGlobAlias::Resolved("Name".into()).render(&ts_indent()),
            "Name"
        );
    }
}
