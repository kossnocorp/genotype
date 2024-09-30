use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::TSImportGlobAlias;

impl GTRender for TSImportGlobAlias {
    fn render(&self, indent: &GTIndent) -> String {
        match self {
            TSImportGlobAlias::Resolved(name) => name.render(indent),

            TSImportGlobAlias::Unresolved => panic!("Tried to render unresolved glob import alias"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{indent::ts_indent, name::TSName};

    #[test]
    fn test_render_resolved() {
        assert_eq!(
            TSImportGlobAlias::Resolved(TSName("Name".to_string())).render(&ts_indent()),
            "Name"
        );
    }
}
