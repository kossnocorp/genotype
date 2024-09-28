use genotype_lang_core::{indent::Indent, node::Node};

use crate::{name::TSName, type_descriptor::TSTypeDescriptor};

pub struct TSProperty {
    pub name: TSName,
    pub descriptor: TSTypeDescriptor,
    pub optional: bool,
}

impl Node for TSProperty {
    fn render(&self, indent: &Indent) -> String {
        format!(
            "{}{}{}: {}",
            indent.string,
            self.name.render(indent),
            if self.optional { "?" } else { "" },
            self.descriptor.render(indent)
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{indent::ts_indent, primitive::TSPrimitive};

    #[test]
    fn test_render_primitive() {
        let indent = ts_indent();
        assert_eq!(
            TSProperty {
                name: TSName("name".to_string()),
                descriptor: TSTypeDescriptor::Primitive(TSPrimitive::String),
                optional: false
            }
            .render(&indent),
            "name: string"
        );
        assert_eq!(
            TSProperty {
                name: TSName("name".to_string()),
                descriptor: TSTypeDescriptor::Name(TSName("Name".to_string())),
                optional: false
            }
            .render(&indent),
            "name: Name"
        );
    }

    #[test]
    fn test_render_indent() {
        let indent = ts_indent().increment();
        assert_eq!(
            TSProperty {
                name: TSName("name".to_string()),
                descriptor: TSTypeDescriptor::Primitive(TSPrimitive::String),
                optional: false
            }
            .render(&indent),
            "  name: string"
        );
    }

    #[test]
    fn test_render_optional() {
        let indent = ts_indent();
        assert_eq!(
            TSProperty {
                name: TSName("name".to_string()),
                descriptor: TSTypeDescriptor::Primitive(TSPrimitive::String),
                optional: true
            }
            .render(&indent),
            "name?: string"
        );
    }
}
