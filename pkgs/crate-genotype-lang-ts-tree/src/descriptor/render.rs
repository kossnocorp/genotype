use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsDescriptor {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        match self {
            TsDescriptor::Array(array) => array.render(state, context),
            TsDescriptor::InlineImport(import) => import.render(state, context),
            TsDescriptor::Intersection(intersection) => intersection.render(state, context),
            TsDescriptor::Literal(literal) => literal.render(state, context),
            TsDescriptor::Primitive(primitive) => primitive.render(state, context),
            TsDescriptor::Reference(name) => name.render(state, context),
            TsDescriptor::Object(object) => object.render(state, context),
            TsDescriptor::Tuple(tuple) => tuple.render(state, context),
            TsDescriptor::Union(union) => union.render(state, context),
            TsDescriptor::Record(record) => record.render(state, context),
            TsDescriptor::Any(any) => any.render(state, context),
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
            TsDescriptor::Array(Box::new(TsArray {
                descriptor: TsDescriptor::Primitive(TsPrimitive::Number)
            }))
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"Array<number>"
        );
    }

    #[test]
    fn test_render_inline_import() {
        assert_snapshot!(
            TsDescriptor::InlineImport(TsInlineImport {
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
            TsDescriptor::Intersection(TsIntersection {
                descriptors: vec![
                    TsObject {
                        properties: vec![TsProperty {
                            doc: None,
                            name: "hello".into(),
                            descriptor: TsPrimitive::String.into(),
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
            TsDescriptor::Object(TsObject {
                properties: vec![
                    TsProperty {
                        doc: None,
                        name: "name".into(),
                        descriptor: TsDescriptor::Primitive(TsPrimitive::String),
                        required: true
                    },
                    TsProperty {
                        doc: None,
                        name: "age".into(),
                        descriptor: TsDescriptor::Primitive(TsPrimitive::Number),
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
            TsDescriptor::Primitive(TsPrimitive::Boolean)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"boolean"
        );
        assert_snapshot!(
            TsDescriptor::Primitive(TsPrimitive::String)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"string"
        );
    }

    #[test]
    fn test_render_reference() {
        assert_snapshot!(
            TsDescriptor::Reference("Name".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Name"
        );
    }

    #[test]
    fn test_render_tuple() {
        assert_snapshot!(
            TsDescriptor::Tuple(TsTuple {
                descriptors: vec![
                    TsDescriptor::Primitive(TsPrimitive::Number),
                    TsDescriptor::Primitive(TsPrimitive::String)
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
            TsDescriptor::Union(TsUnion {
                descriptors: vec![
                    TsDescriptor::Primitive(TsPrimitive::String),
                    TsDescriptor::Primitive(TsPrimitive::Number),
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
            TsDescriptor::Record(Box::new(TsRecord {
                key: TsRecordKey::String,
                descriptor: TsDescriptor::Primitive(TsPrimitive::Number),
            }))
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"Record<string, number>"
        );
    }

    #[test]
    fn test_render_any() {
        assert_snapshot!(
            TsDescriptor::Any(TsAny)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"any"
        );
    }
}
