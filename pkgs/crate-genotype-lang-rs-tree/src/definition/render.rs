use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for RsDefinition {
    type RenderState = RsRenderState;

    type RenderContext = RsRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        Ok(match self {
            RsDefinition::Alias(alias) => alias.render(state, context)?,
            RsDefinition::Struct(interface) => interface.render(state, context)?,
            RsDefinition::Enum(r#enum) => r#enum.render(state, context)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_alias() {
        assert_snapshot!(
            RsDefinition::Alias(RsAlias {
                id: GtDefinitionId("module".into(), "Name".into()),
                doc: None,
                name: "Name".into(),
                descriptor: RsDescriptor::Primitive(RsPrimitive::String),
            })
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"pub type Name = String;"
        );
    }

    #[test]
    fn test_render_struct() {
        assert_snapshot!(
            RsDefinition::Struct(RsStruct {
                id: GtDefinitionId("module".into(), "Name".into()),
                doc: None,
                attributes: vec![],
                name: "Name".into(),
                fields: vec![
                    RsField {
                        doc: None,
                        attributes: vec![],
                        name: "name".into(),
                        descriptor: RsDescriptor::Primitive(RsPrimitive::String),
                    },
                    RsField {
                        doc: None,
                        attributes: vec![],
                        name: "age".into(),
                        descriptor: RsDescriptor::Primitive(RsPrimitive::IntSize),
                    }
                ]
                .into(),
            })
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"
        pub struct Name {
            pub name: String,
            pub age: isize,
        }
        "
        );
    }

    #[test]
    fn test_render_enum() {
        assert_snapshot!(
            RsDefinition::Enum(RsEnum {
                id: GtDefinitionId("module".into(), "ValuesUnion".into()),
                doc: None,
                attributes: vec![],
                name: "ValuesUnion".into(),
                variants: vec![
                    RsEnumVariant {
                        doc: None,
                        name: "Boolean".into(),
                        attributes: vec![],
                        descriptor: Some(RsEnumVariantDescriptor::Descriptor(
                            RsDescriptor::Primitive(RsPrimitive::Boolean).into()
                        )),
                    },
                    RsEnumVariant {
                        doc: None,
                        name: "String".into(),
                        attributes: vec![],
                        descriptor: Some(RsEnumVariantDescriptor::Descriptor(
                            RsDescriptor::Primitive(RsPrimitive::String).into()
                        )),
                    }
                ],
            })
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"
        pub enum ValuesUnion {
            Boolean(bool),
            String(String),
        }
        "
        );
    }
}
