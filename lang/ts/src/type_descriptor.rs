use crate::{name::TSName, primitive::TSPrimitive};
use genotype_lang_core::{indent::Indent, node::Node};

pub enum TSTypeDescriptor {
    Primitive(TSPrimitive),
    Name(TSName),
}

impl Node for TSTypeDescriptor {
    fn render(&self, indent: &Indent) -> String {
        match self {
            TSTypeDescriptor::Primitive(primitive) => primitive.render(indent),
            TSTypeDescriptor::Name(name) => name.render(indent),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{indent::ts_indent, name::TSName, primitive::TSPrimitive};

    #[test]
    fn test_render_primitive() {
        let indent = ts_indent();
        assert_eq!(
            TSTypeDescriptor::Primitive(TSPrimitive::Boolean).render(&indent),
            "boolean"
        );
        assert_eq!(
            TSTypeDescriptor::Primitive(TSPrimitive::String).render(&indent),
            "string"
        );
    }

    #[test]
    fn test_render_name() {
        let indent = ts_indent();
        assert_eq!(
            TSTypeDescriptor::Name(TSName("Name".to_string())).render(&indent),
            "Name"
        );
    }
}
