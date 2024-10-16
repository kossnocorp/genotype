use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::PYImport;

impl GTRender for PYImport {
    fn render(&self, indent: &GTIndent) -> String {
        format!(
            r#"import {} from "{}";"#,
            self.reference.render(indent),
            self.path.render(indent)
        )
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::*;

    #[test]
    fn test_render_default() {
        assert_eq!(
            PYImport {
                path: "../path/to/module.ts".into(),
                reference: PYImportReference::Default("Name".into()),
            }
            .render(&py_indent()),
            r#"import Name from "../path/to/module.ts";"#
        );
    }

    #[test]
    fn test_render_glob() {
        assert_eq!(
            PYImport {
                path: "../path/to/module.ts".into(),
                reference: PYImportReference::Glob("name".into()),
            }
            .render(&py_indent()),
            r#"import * as name from "../path/to/module.ts";"#
        );
    }

    #[test]
    fn test_render_named() {
        assert_eq!(
            PYImport {
                path: "../path/to/module.ts".into(),
                reference: PYImportReference::Named(vec![
                    PYImportName::Name("Name".into()),
                    PYImportName::Alias("Name".into(), "Alias".into()),
                ])
            }
            .render(&py_indent()),
            r#"import { Name, Name as Alias } from "../path/to/module.ts";"#
        );
    }
}
