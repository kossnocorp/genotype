use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for TSDescriptor {
    type RenderContext = TSRenderContext<'a>;

    fn render(&self, context: &mut Self::RenderContext) -> Result<String> {
        match self {
            TSDescriptor::Array(array) => array.render(context),
            TSDescriptor::InlineImport(import) => import.render(context),
            TSDescriptor::Intersection(intersection) => intersection.render(context),
            TSDescriptor::Literal(literal) => literal.render(context),
            TSDescriptor::Primitive(primitive) => primitive.render(context),
            TSDescriptor::Reference(name) => name.render(context),
            TSDescriptor::Object(object) => object.render(context),
            TSDescriptor::Tuple(tuple) => tuple.render(context),
            TSDescriptor::Union(union) => union.render(context),
            TSDescriptor::Record(record) => record.render(context),
            TSDescriptor::Any(any) => any.render(context),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render_array() {
        assert_eq!(
            TSDescriptor::Array(Box::new(TSArray {
                descriptor: TSDescriptor::Primitive(TSPrimitive::Number)
            }))
            .render(&mut Default::default())
            .unwrap(),
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
            .render(&mut Default::default())
            .unwrap(),
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
            .render(&mut Default::default())
            .unwrap(),
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
            .render(&mut Default::default())
            .unwrap(),
            r#"{
  name: string,
  age?: number
}"#
        );
    }

    #[test]
    fn test_render_primitive() {
        assert_eq!(
            TSDescriptor::Primitive(TSPrimitive::Boolean)
                .render(&mut Default::default())
                .unwrap(),
            "boolean"
        );
        assert_eq!(
            TSDescriptor::Primitive(TSPrimitive::String)
                .render(&mut Default::default())
                .unwrap(),
            "string"
        );
    }

    #[test]
    fn test_render_reference() {
        assert_eq!(
            TSDescriptor::Reference("Name".into())
                .render(&mut Default::default())
                .unwrap(),
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
            .render(&mut Default::default())
            .unwrap(),
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
            .render(&mut Default::default())
            .unwrap(),
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
            .render(&mut Default::default())
            .unwrap(),
            "Record<string, number>"
        );
    }

    #[test]
    fn test_render_any() {
        assert_eq!(
            TSDescriptor::Any(TSAny)
                .render(&mut Default::default())
                .unwrap(),
            "any"
        );
    }
}
