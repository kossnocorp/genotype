use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::TSDescriptor;

impl GTRender for TSDescriptor {
    fn render(&self, indent: &GTIndent) -> String {
        match self {
            TSDescriptor::Array(array) => array.render(indent),
            TSDescriptor::InlineImport(import) => import.render(indent),
            TSDescriptor::Intersection(intersection) => intersection.render(indent),
            TSDescriptor::Literal(literal) => literal.render(indent),
            TSDescriptor::Primitive(primitive) => primitive.render(indent),
            TSDescriptor::Reference(name) => name.render(indent),
            TSDescriptor::Object(object) => object.render(indent),
            TSDescriptor::Tuple(tuple) => tuple.render(indent),
            TSDescriptor::Union(union) => union.render(indent),
            TSDescriptor::Record(record) => record.render(indent),
            TSDescriptor::Any(any) => any.render(indent),
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::*;

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
    fn test_render_inline_import() {
        assert_eq!(
            TSDescriptor::InlineImport(TSInlineImport {
                path: "../path/to/module.ts".into(),
                name: "Name".into(),
            })
            .render(&ts_indent()),
            r#"import("../path/to/module.ts").Name"#
        );
    }

    #[test]
    fn test_render_intersection() {
        assert_eq!(
            TSDescriptor::Intersection(TSIntersection {
                descriptors: vec![
                    TSObject {
                        properties: vec![TSProperty {
                            doc: None,
                            name: "hello".into(),
                            descriptor: TSPrimitive::String.into(),
                            required: true,
                        }],
                    }
                    .into(),
                    "World".into(),
                ]
            })
            .render(&ts_indent()),
            r#"{
  hello: string
} & World"#
        );
    }

    #[test]
    fn test_render_object() {
        assert_eq!(
            TSDescriptor::Object(TSObject {
                properties: vec![
                    TSProperty {
                        doc: None,
                        name: "name".into(),
                        descriptor: TSDescriptor::Primitive(TSPrimitive::String),
                        required: true
                    },
                    TSProperty {
                        doc: None,
                        name: "age".into(),
                        descriptor: TSDescriptor::Primitive(TSPrimitive::Number),
                        required: false
                    }
                ]
            })
            .render(&ts_indent()),
            r#"{
  name: string,
  age?: number
}"#
        );
    }

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
            TSDescriptor::Reference("Name".into()).render(&ts_indent()),
            "Name"
        );
    }

    #[test]
    fn test_render_tuple() {
        assert_eq!(
            TSDescriptor::Tuple(TSTuple {
                descriptors: vec![
                    TSDescriptor::Primitive(TSPrimitive::Number),
                    TSDescriptor::Primitive(TSPrimitive::String)
                ]
            })
            .render(&ts_indent()),
            "[number, string]"
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
    fn test_render_record() {
        assert_eq!(
            TSDescriptor::Record(Box::new(TSRecord {
                key: TSRecordKey::String,
                descriptor: TSDescriptor::Primitive(TSPrimitive::Number),
            }))
            .render(&ts_indent()),
            "Record<string, number>"
        );
    }

    #[test]
    fn test_render_any() {
        assert_eq!(TSDescriptor::Any(TSAny).render(&ts_indent()), "any");
    }
}
