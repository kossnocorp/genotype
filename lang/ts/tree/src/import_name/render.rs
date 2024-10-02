use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::TSImportName;

impl GTRender for TSImportName {
    fn render(&self, indent: &GTIndent) -> String {
        match self {
            TSImportName::Name(name) => name.render(indent),
            TSImportName::Alias(name, alias) => {
                format!("{} as {}", name.render(indent), alias.render(indent))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indent::ts_indent;

    #[test]
    fn test_render_name() {
        assert_eq!(
            TSImportName::Name("Name".into()).render(&ts_indent()),
            "Name"
        );
    }

    #[test]
    fn test_render_alias() {
        assert_eq!(
            TSImportName::Alias("Name".into(), "Alias".into()).render(&ts_indent()),
            "Name as Alias"
        );
    }
}
