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
    use crate::{indent::ts_indent, name::TSName};

    #[test]
    fn test_render_name() {
        let indent = ts_indent();
        assert_eq!(
            TSImportName::Name(TSName("Name".to_string())).render(&indent),
            "Name"
        );
    }

    #[test]
    fn test_render_alias() {
        let indent = ts_indent();
        assert_eq!(
            TSImportName::Alias(TSName("Name".to_string()), TSName("Alias".to_string()))
                .render(&indent),
            "Name as Alias"
        );
    }
}
