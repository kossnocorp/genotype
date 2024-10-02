use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::TSDescriptor;

impl GTRender for TSDescriptor {
    fn render(&self, indent: &GTIndent) -> String {
        match self {
            TSDescriptor::Primitive(primitive) => primitive.render(indent),
            TSDescriptor::Reference(name) => name.render(indent),
            TSDescriptor::Union(union) => union.render(indent),
            TSDescriptor::Object(object) => object.render(indent),
            TSDescriptor::Array(array) => array.render(indent),
            TSDescriptor::Tuple(tuple) => tuple.render(indent),
            TSDescriptor::InlineImport(import) => import.render(indent),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        array::TSArray, indent::ts_indent, inline_import::TSInlineImport, object::TSObject,
        path::TSPath, primitive::TSPrimitive, property::TSProperty, reference::TSReference,
        tuple::TSTuple, union::TSUnion,
    };

    #[test]
    fn test_render_primitive() {
        assert_eq!(
            TSDescriptor::Primitive(TSPrimitive::Boolean).render(&ts_indent()),
            "boolean"
        );
        assert_eq!(
            TSDescriptor::Primitive(TSPrimitive::String).render(&ts_indent()),
            "string"
        );
    }

    #[test]
    fn test_render_reference() {
        assert_eq!(
            TSDescriptor::Reference(TSReference::Unresolved("Name".into())).render(&ts_indent()),
            "Name"
        );
    }

    #[test]
    fn test_render_union() {
        assert_eq!(
            TSDescriptor::Union(TSUnion {
                descriptors: vec![
                    TSDescriptor::Primitive(TSPrimitive::String),
                    TSDescriptor::Primitive(TSPrimitive::Number),
                ]
            })
            .render(&ts_indent()),
            "string | number"
        );
    }

    #[test]
    fn test_render_object() {
        assert_eq!(
            TSDescriptor::Object(Box::new(TSObject {
                properties: vec![
                    TSProperty {
                        name: "name".into(),
                        descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                        required: true
                    },
                    TSProperty {
                        name: "age".into(),
                        descriptor: TSDescriptor::Primitive(TSPrimitive::Number),
                        required: false
                    }
                ]
            }))
            .render(&ts_indent()),
            r#"{
  name: string,
  age?: number
}"#
        );
    }

    #[test]
    fn test_render_array() {
        assert_eq!(
            TSDescriptor::Array(Box::new(TSArray {
                descriptor: TSDescriptor::Primitive(TSPrimitive::Number)
            }))
            .render(&ts_indent()),
            "Array<number>"
        );
    }

    #[test]
    fn test_render_tuple() {
        assert_eq!(
            TSDescriptor::Tuple(Box::new(TSTuple {
                descriptors: vec![
                    TSDescriptor::Primitive(TSPrimitive::Number),
                    TSDescriptor::Primitive(TSPrimitive::String)
                ]
            }))
            .render(&ts_indent()),
            "[number, string]"
        );
    }

    #[test]
    fn test_render_import_type() {
        assert_eq!(
            TSDescriptor::InlineImport(TSInlineImport {
                path: TSPath::Resolved("../path/to/module.ts".into()),
                name: "Name".into(),
            })
            .render(&ts_indent()),
            r#"import("../path/to/module.ts").Name"#
        );
    }
}
