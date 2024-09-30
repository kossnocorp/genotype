use genotype_lang_core::{indent::Indent, node::Node};

use crate::{import_name::TSImportName, name::TSName};

#[derive(Debug, PartialEq, Clone)]
pub enum TSImportReference {
    Default(TSName),
    Glob(TSName),
    Named(Vec<TSImportName>),
}

impl Node for TSImportReference {
    fn render(&self, indent: &Indent) -> String {
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
    use crate::{indent::ts_indent, name::TSName};

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
            TSImportReference::Glob(TSName("Name".to_string())).render(&indent),
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
