use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::TSImport;

impl GTRender for TSImport {
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
    use crate::import_glob_alias::TSImportGlobAlias;
    use crate::import_name::TSImportName;
    use crate::import_reference::TSImportReference;
    use crate::indent::ts_indent;
    use crate::path::TSPath;

    #[test]
    fn test_render_default() {
        assert_eq!(
            TSImport {
                path: TSPath::Resolved("../path/to/module.ts".into()),
                reference: TSImportReference::Default("Name".into()),
            }
            .render(&ts_indent()),
            r#"import Name from "../path/to/module.ts";"#
        );
    }

    #[test]
    fn test_render_glob() {
        assert_eq!(
            TSImport {
                path: TSPath::Resolved("../path/to/module.ts".into()),
                reference: TSImportReference::Glob(TSImportGlobAlias::Resolved("Name".into())),
            }
            .render(&ts_indent()),
            r#"import * as Name from "../path/to/module.ts";"#
        );
    }

    #[test]
    fn test_render_named() {
        assert_eq!(
            TSImport {
                path: TSPath::Resolved("../path/to/module.ts".into()),
                reference: TSImportReference::Named(vec![
                    TSImportName::Name("Name".into()),
                    TSImportName::Alias("Name".into(), "Alias".into()),
                ])
            }
            .render(&ts_indent()),
            r#"import { Name, Name as Alias } from "../path/to/module.ts";"#
        );
    }
}
