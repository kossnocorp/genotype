use crate::{name::TSName, type_descriptor::TSTypeDescriptor};
use genotype_lang_core::{indent::Indent, node::Node};

pub struct TSAlias {
    pub name: TSName,
    pub descriptor: TSTypeDescriptor,
}

impl Node for TSAlias {
    fn render(&self, indent: &Indent) -> String {
        format!(
            "type {} = {};",
            self.name.render(indent),
            self.descriptor.render(indent)
        )
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{indent::ts_indent, name::TSName, primitive::TSPrimitive};

    #[test]
    fn test_render() {
        let indent = ts_indent();
        assert_eq!(
            TSAlias {
                name: TSName("Name".to_string()),
                descriptor: TSTypeDescriptor::Primitive(TSPrimitive::String)
            }
            .render(&indent),
            "type Name = string;"
        );
        assert_eq!(
            TSTypeDescriptor::Primitive(TSPrimitive::String).render(&indent),
            "string"
        );
    }
}
