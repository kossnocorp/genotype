use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::TSImportReference;

impl GTRender for TSImportReference {
    fn render(&self, indent: &GTIndent) -> String {
        match self {
            TSImportReference::Default(name) => name.clone(),

            TSImportReference::Glob(name) => format!("* as {}", name.render(indent)),

            TSImportReference::Named(names) => {
                let names = names
                    .iter()
                    .map(|name| name.render(indent))
                    .collect::<Vec<String>>()
                    .join(", ");
                format!("{{ {} }}", names)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        import_glob_alias::TSImportGlobAlias, import_name::TSImportName, indent::ts_indent,
    };

    #[test]
    fn test_render_default() {
        assert_eq!(
            TSImportReference::Default("Name".into()).render(&ts_indent()),
            "Name"
        );
    }

    #[test]
    fn test_render_glob() {
        assert_eq!(
            TSImportReference::Glob(TSImportGlobAlias::Resolved("Name".into()))
                .render(&ts_indent()),
            "* as Name"
        );
    }

    #[test]
    fn test_render_named() {
        assert_eq!(
            TSImportReference::Named(vec![
                TSImportName::Name("Name".into()),
                TSImportName::Alias("Name".into(), "Alias".into()),
            ])
            .render(&ts_indent()),
            "{ Name, Name as Alias }"
        );
    }
}
