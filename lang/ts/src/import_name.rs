use genotype_lang_core::{indent::Indent, node::Node};

use crate::name::TSName;

#[derive(Debug, PartialEq, Clone)]
pub enum TSImportName {
    Name(TSName),
    Alias(TSName, TSName),
}

impl Node for TSImportName {
    fn render(&self, indent: &Indent) -> String {
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
    use crate::{
        alias::TSAlias, indent::ts_indent, interface::TSInterface, name::TSName,
        primitive::TSPrimitive, property::TSProperty, type_descriptor::TSTypeDescriptor,
    };

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
