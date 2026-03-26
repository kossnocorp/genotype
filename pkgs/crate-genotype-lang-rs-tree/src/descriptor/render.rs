use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for RsDescriptor {
    type RenderState = RsRenderState;

    type RenderContext = RsRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        Ok(match self {
            RsDescriptor::Box(descriptor) => {
                format!("Box<{}>", descriptor.render(state, context)?)
            }
            RsDescriptor::Enum(r#enum) => r#enum.render(state, context)?,
            RsDescriptor::Vec(array) => array.render(state, context)?,
            RsDescriptor::Primitive(primitive) => primitive.render(state, context)?,
            RsDescriptor::Reference(name) => name.render(state, context)?,
            RsDescriptor::InlineUse(inline_use) => inline_use.render(state, context)?,
            RsDescriptor::Tuple(tuple) => tuple.render(state, context)?,
            RsDescriptor::Map(dict) => dict.render(state, context)?,
            RsDescriptor::Option(option) => option.render(state, context)?,
            RsDescriptor::Any(any) => any.render(state, context)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_array() {
        assert_snapshot!(
            RsDescriptor::Vec(Box::new(RsVec {
                descriptor: RsDescriptor::Primitive(RsPrimitive::IntSize)
            }))
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"Vec<isize>"
        );
    }

    #[test]
    fn test_render_box() {
        assert_snapshot!(
            RsDescriptor::boxed(RsDescriptor::Primitive(RsPrimitive::String))
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Box<String>"
        );
    }

    #[test]
    fn test_render_primitive() {
        assert_snapshot!(
            RsDescriptor::Primitive(RsPrimitive::Boolean)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"bool"
        );
        assert_snapshot!(
            RsDescriptor::Primitive(RsPrimitive::String)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"String"
        );
    }

    #[test]
    fn test_render_reference() {
        assert_snapshot!(
            RsDescriptor::Reference(RsReference {
                id: GtReferenceId("module".into(), (0, 0).into()),
                identifier: "Name".into(),
                definition_id: GtDefinitionId("module".into(), "Name".into())
            })
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"Name"
        );
    }

    #[test]
    fn test_render_inline_use() {
        assert_snapshot!(
            RsDescriptor::InlineUse(RsInlineUse {
                path: RsPath("path/to/module".into(), "self::path::to::module".into()),
                name: "Name".into()
            })
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"self::path::to::module::Name"
        );
    }

    #[test]
    fn test_render_tuple() {
        assert_snapshot!(
            RsDescriptor::Tuple(RsTuple {
                descriptors: vec![
                    RsDescriptor::Primitive(RsPrimitive::IntSize),
                    RsDescriptor::Primitive(RsPrimitive::String)
                ]
            })
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"(isize, String)"
        );
    }

    #[test]
    fn test_render_map() {
        assert_snapshot!(
            RsDescriptor::Map(Box::new(RsMap {
                key: RsPrimitive::String.into(),
                descriptor: RsPrimitive::IntSize.into(),
            }))
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"BTreeMap<String, isize>"
        );
    }

    #[test]
    fn test_render_option() {
        assert_snapshot!(
            RsDescriptor::Option(Box::new(RsOption::new(RsDescriptor::Primitive(
                RsPrimitive::String
            ))))
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"Option<String>"
        );
    }

    #[test]
    fn test_render_any() {
        assert_snapshot!(
            RsDescriptor::Any(RsAny)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            @"Any"
        );
    }
}
