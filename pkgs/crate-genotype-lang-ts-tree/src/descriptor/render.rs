use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TSDescriptor {
    type RenderState = TSRenderState;

    type RenderContext = TSRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        match self {
            TSDescriptor::Array(array) => array.render(state, context),
            TSDescriptor::InlineImport(import) => import.render(state, context),
            TSDescriptor::Intersection(intersection) => intersection.render(state, context),
            TSDescriptor::Literal(literal) => literal.render(state, context),
            TSDescriptor::Primitive(primitive) => primitive.render(state, context),
            TSDescriptor::Reference(name) => name.render(state, context),
            TSDescriptor::Object(object) => object.render(state, context),
            TSDescriptor::Tuple(tuple) => tuple.render(state, context),
            TSDescriptor::Union(union) => union.render(state, context),
            TSDescriptor::Record(record) => record.render(state, context),
            TSDescriptor::Any(any) => any.render(state, context),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_array() {
        assert_snapshot!(
            TSDescriptor::Array(Box::new(TSArray {
                descriptor: TSDescriptor::Primitive(TSPrimitive::Number)
            }))
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"Array<number>"
        );
    }

    #[test]
    fn test_render_inline_import() {
        assert_snapshot!(
            TSDescriptor::InlineImport(TSInlineImport {
                path: "../path/to/module".into(),
                name: "Name".into(),
            })
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @r#"import("../path/to/module.js").Name"#
        );
    }

    #[test]
    fn test_render_intersection() {
        assert_snapshot!(
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
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"
        {
          hello: string
        } & World
        "
        );
    }

    #[test]
    fn test_render_object() {
        assert_snapshot!(
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
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"
        {
          name: string,
          age?: number
        }
        "
        );
    }

    #[test]
    fn test_render_primitive() {
        assert_snapshot!(
            TSDescriptor::Primitive(TSPrimitive::Boolean)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"boolean"
        );
        assert_snapshot!(
            TSDescriptor::Primitive(TSPrimitive::String)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"string"
        );
    }

    #[test]
    fn test_render_reference() {
        assert_snapshot!(
            TSDescriptor::Reference("Name".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Name"
        );
    }

    #[test]
    fn test_render_tuple() {
        assert_snapshot!(
            TSDescriptor::Tuple(TSTuple {
                descriptors: vec![
                    TSDescriptor::Primitive(TSPrimitive::Number),
                    TSDescriptor::Primitive(TSPrimitive::String)
                ]
            })
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"[number, string]"
        );
    }

    #[test]
    fn test_render_union() {
        assert_snapshot!(
            TSDescriptor::Union(TSUnion {
                descriptors: vec![
                    TSDescriptor::Primitive(TSPrimitive::String),
                    TSDescriptor::Primitive(TSPrimitive::Number),
                ]
            })
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"string | number"
        );
    }

    #[test]
    fn test_render_record() {
        assert_snapshot!(
            TSDescriptor::Record(Box::new(TSRecord {
                key: TSRecordKey::String,
                descriptor: TSDescriptor::Primitive(TSPrimitive::Number),
            }))
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"Record<string, number>"
        );
    }

    #[test]
    fn test_render_any() {
        assert_snapshot!(
            TSDescriptor::Any(TSAny)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"any"
        );
    }
}
