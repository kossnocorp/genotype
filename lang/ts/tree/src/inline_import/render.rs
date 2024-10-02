use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::TSInlineImport;

impl GTRender for TSInlineImport {
    fn render(&self, indent: &GTIndent) -> String {
        format!(
            r#"import("{}").{}"#,
            self.path.render(indent),
            self.name.render(indent)
        )
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::{indent::ts_indent, path::TSPath};

    #[test]
    fn test_render() {
        assert_eq!(
            TSInlineImport {
                path: TSPath::Resolved("./path/to/module.ts".into()),
                name: "Name".into(),
            }
            .render(&ts_indent()),
            r#"import("./path/to/module.ts").Name"#
        );
    }
}
