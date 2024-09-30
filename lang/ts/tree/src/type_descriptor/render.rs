use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::TSTypeDescriptor;

impl GTRender for TSTypeDescriptor {
    fn render(&self, indent: &GTIndent) -> String {
        match self {
            TSTypeDescriptor::Primitive(primitive) => primitive.render(indent),
            TSTypeDescriptor::Name(name) => name.render(indent),
            TSTypeDescriptor::Union(union) => union.render(indent),
            TSTypeDescriptor::Array(array) => array.render(indent),
            TSTypeDescriptor::Tuple(tuple) => tuple.render(indent),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        array::TSArray, indent::ts_indent, name::TSName, primitive::TSPrimitive, tuple::TSTuple,
        union::TSUnion,
    };

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

    #[test]
    fn test_render_union() {
        let indent = ts_indent();
        assert_eq!(
            TSTypeDescriptor::Union(TSUnion {
                descriptors: vec![
                    TSTypeDescriptor::Primitive(TSPrimitive::String),
                    TSTypeDescriptor::Primitive(TSPrimitive::Number),
                ]
            })
            .render(&indent),
            "string | number"
        );
    }

    #[test]
    fn test_render_array() {
        let indent = ts_indent();
        assert_eq!(
            TSTypeDescriptor::Array(Box::new(TSArray {
                descriptor: TSTypeDescriptor::Primitive(TSPrimitive::Number)
            }))
            .render(&indent),
            "Array<number>"
        );
    }

    #[test]
    fn test_render_tuple() {
        let indent = ts_indent();
        assert_eq!(
            TSTypeDescriptor::Tuple(Box::new(TSTuple {
                descriptors: vec![
                    TSTypeDescriptor::Primitive(TSPrimitive::Number),
                    TSTypeDescriptor::Primitive(TSPrimitive::String)
                ]
            }))
            .render(&indent),
            "[number, string]"
        );
    }
}