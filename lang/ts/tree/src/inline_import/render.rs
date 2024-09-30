use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::TSInlineImport;

impl GTRender for TSInlineImport {
    fn render(&self, indent: &GTIndent) -> String {
        format!(r#"import("{}").{}"#, self.path, self.name.render(indent),)
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::import_name::TSImportName;
    use crate::import_reference::TSImportReference;
    use crate::indent::ts_indent;
    use crate::name::TSName;

    #[test]
    fn test_render() {
        let indent = ts_indent();
        assert_eq!(
            TSInlineImport {
                path: "../path/to/module.ts".to_string(),
                name: TSName("Name".to_string()),
            }
            .render(&indent),
            r#"import("../path/to/module.ts").Name"#
        );
    }
}
