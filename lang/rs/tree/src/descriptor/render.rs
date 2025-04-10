use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for RSDescriptor {
    type RenderContext = RSRenderContext<'a>;

    fn render(&self, context: &mut Self::RenderContext) -> Result<String> {
        Ok(match self {
            RSDescriptor::Enum(r#enum) => r#enum.render(context)?,
            RSDescriptor::Vec(array) => array.render(context)?,
            RSDescriptor::Primitive(primitive) => primitive.render(context)?,
            RSDescriptor::Reference(name) => name.render(context)?,
            RSDescriptor::InlineUse(inline_use) => inline_use.render(context)?,
            RSDescriptor::Tuple(tuple) => tuple.render(context)?,
            RSDescriptor::Map(dict) => dict.render(context)?,
            RSDescriptor::Option(option) => option.render(context)?,
            RSDescriptor::Any(any) => any.render(context)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use genotype_parser::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render_array() {
        assert_eq!(
            RSDescriptor::Vec(Box::new(RSVec {
                descriptor: RSDescriptor::Primitive(RSPrimitive::IntSize)
            }))
            .render(&mut Default::default())
            .unwrap(),
            "Vec<isize>"
        );
    }

    #[test]
    fn test_render_primitive() {
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::Boolean)
                .render(&mut Default::default())
                .unwrap(),
            "bool"
        );
        assert_eq!(
            RSDescriptor::Primitive(RSPrimitive::String)
                .render(&mut Default::default())
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
            .render(&mut Default::default())
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
            .render(&mut Default::default())
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
            .render(&mut Default::default())
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
            .render(&mut Default::default())
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
            .render(&mut Default::default())
            .unwrap(),
            "Option<String>"
        );
    }

    #[test]
    fn test_render_any() {
        assert_eq!(
            RSDescriptor::Any(RSAny)
                .render(&mut Default::default())
                .unwrap(),
            "Any"
        );
    }
}
