use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::TSImportReference;

impl GTRender for TSImportReference {
    fn render(&self, indent: &GTIndent) -> String {
        match self {
            TSImportReference::Default(name) => name.clone(),

            TSImportReference::Glob(name) => format!("* as {}", name),

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
    use crate::*;

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
            TSImportReference::Glob("name".into()).render(&ts_indent()),
            "* as name"
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
