use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::TSImport;

impl GTRender for TSImport {
    fn render(&self, indent: &GTIndent) -> String {
        format!(
            r#"import {} from "{}";"#,
            self.reference.render(indent),
            self.path
        )
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::import_glob_alias::TSImportGlobAlias;
    use crate::import_name::TSImportName;
    use crate::import_reference::TSImportReference;
    use crate::indent::ts_indent;
    use crate::name::TSName;

    #[test]
    fn test_render_default() {
        let indent = ts_indent();
        assert_eq!(
            TSImport {
                path: "../path/to/module.ts".to_string(),
                reference: TSImportReference::Default(TSName("Name".to_string())),
            }
            .render(&indent),
            r#"import Name from "../path/to/module.ts";"#
        );
    }

    #[test]
    fn test_render_glob() {
        let indent = ts_indent();
        assert_eq!(
            TSImport {
                path: "../path/to/module.ts".to_string(),
                reference: TSImportReference::Glob(TSImportGlobAlias::Resolved(TSName(
                    "Name".to_string()
                ))),
            }
            .render(&indent),
            r#"import * as Name from "../path/to/module.ts";"#
        );
    }

    #[test]
    fn test_render_named() {
        let indent = ts_indent();
        assert_eq!(
            TSImport {
                path: "../path/to/module.ts".to_string(),
                reference: TSImportReference::Named(vec![
                    TSImportName::Name(TSName("Name".to_string())),
                    TSImportName::Alias(TSName("Name".to_string()), TSName("Alias".to_string())),
                ])
            }
            .render(&indent),
            r#"import { Name, Name as Alias } from "../path/to/module.ts";"#
        );
    }
}
