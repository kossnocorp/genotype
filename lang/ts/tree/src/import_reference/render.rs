use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::TSImportReference;

impl GTRender for TSImportReference {
    fn render(&self, indent: &GTIndent) -> String {
        match self {
            TSImportReference::Default(name) => name.render(indent),

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
        name::TSName,
    };

    #[test]
    fn test_render_default() {
        let indent = ts_indent();
        assert_eq!(
            TSImportReference::Default(TSName("Name".to_string())).render(&indent),
            "Name"
        );
    }

    #[test]
    fn test_render_glob() {
        let indent = ts_indent();
        assert_eq!(
            TSImportReference::Glob(TSImportGlobAlias::Resolved(TSName("Name".to_string())))
                .render(&indent),
            "* as Name"
        );
    }

    #[test]
    fn test_render_named() {
        let indent = ts_indent();
        assert_eq!(
            TSImportReference::Named(vec![
                TSImportName::Name(TSName("Name".to_string())),
                TSImportName::Alias(TSName("Name".to_string()), TSName("Alias".to_string())),
            ])
            .render(&indent),
            "{ Name, Name as Alias }"
        );
    }
}
