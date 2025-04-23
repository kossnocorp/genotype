use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for RSDescriptor {
    type RenderState = RSRenderState;

    type RenderContext = RSRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        Ok(match self {
            RSDescriptor::Enum(r#enum) => r#enum.render(state, context)?,
            RSDescriptor::Vec(array) => array.render(state, context)?,
            RSDescriptor::Primitive(primitive) => primitive.render(state, context)?,
            RSDescriptor::Reference(name) => name.render(state, context)?,
            RSDescriptor::InlineUse(inline_use) => inline_use.render(state, context)?,
            RSDescriptor::Tuple(tuple) => tuple.render(state, context)?,
            RSDescriptor::Map(dict) => dict.render(state, context)?,
            RSDescriptor::Option(option) => option.render(state, context)?,
            RSDescriptor::Any(any) => any.render(state, context)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render_array() {
        assert_eq!(
            RSDescriptor::Vec(Box::new(RSVec {
                descriptor: RSDescriptor::Primitive(RSPrimitive::IntSize)
            }))
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            "Vec<isize>"
        );
    }

    #[test]
    fn test_render_primitive() {
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::Boolean)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            "bool"
        );
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::String)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            "String"
        );
    }

    #[test]
    fn test_render_reference() {
        assert_eq!(
            RSDescriptor::Reference(RSReference {
                id: GTReferenceId("module".into(), (0, 0).into()),
                identifier: "Name".into(),
                definition_id: GTDefinitionId("module".into(), "Name".into())
            })
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            "Name"
        );
    }

    #[test]
    fn test_render_inline_use() {
        assert_eq!(
            RSDescriptor::InlineUse(RSInlineUse {
                path: RSPath("path/to/module".into(), "self::path::to::module".into()),
                name: "Name".into()
            })
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            "self::path::to::module::Name"
        );
    }

    #[test]
    fn test_render_tuple() {
        assert_eq!(
            RSDescriptor::Tuple(RSTuple {
                descriptors: vec![
                    RSDescriptor::Primitive(RSPrimitive::IntSize),
                    RSDescriptor::Primitive(RSPrimitive::String)
                ]
            })
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            "(isize, String)"
        );
    }

    #[test]
    fn test_render_map() {
        assert_eq!(
            RSDescriptor::Map(Box::new(RSMap {
                key: RSPrimitive::String.into(),
                descriptor: RSPrimitive::IntSize.into(),
            }))
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            "BTreeMap<String, isize>"
        );
    }

    #[test]
    fn test_render_option() {
        assert_eq!(
            RSDescriptor::Option(Box::new(RSOption::new(RSDescriptor::Primitive(
                RSPrimitive::String
            ))))
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            "Option<String>"
        );
    }

    #[test]
    fn test_render_any() {
        assert_eq!(
            RSDescriptor::Any(RSAny)
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            "Any"
        );
    }
}
