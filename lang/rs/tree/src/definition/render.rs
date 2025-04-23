use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for RSDefinition {
    type RenderState = RSRenderState;

    type RenderContext = RSRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        Ok(match self {
            RSDefinition::Alias(alias) => alias.render(state, context)?,
            RSDefinition::Struct(interface) => interface.render(state, context)?,
            RSDefinition::Enum(r#enum) => r#enum.render(state, context)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render_alias() {
        assert_eq!(
            RSDefinition::Alias(RSAlias {
                id: GTDefinitionId("module".into(), "Name".into()),
                doc: None,
                name: "Name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
            })
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            "pub type Name = String;"
        );
    }

    #[test]
    fn test_render_struct() {
        assert_eq!(
            RSDefinition::Struct(RSStruct {
                id: GTDefinitionId("module".into(), "Name".into()),
                doc: None,
                attributes: vec![],
                name: "Name".into(),
                fields: vec![
                    RSField {
                        doc: None,
                        attributes: vec![],
                        name: "name".into(),
                        descriptor: RSDescriptor::Primitive(RSPrimitive::String),
                    },
                    RSField {
                        doc: None,
                        attributes: vec![],
                        name: "age".into(),
                        descriptor: RSDescriptor::Primitive(RSPrimitive::IntSize),
                    }
                ]
                .into(),
            })
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            r#"pub struct Name {
    pub name: String,
    pub age: isize,
}"#
        );
    }

    #[test]
    fn test_render_enum() {
        assert_eq!(
            RSDefinition::Enum(RSEnum {
                id: GTDefinitionId("module".into(), "ValuesUnion".into()),
                doc: None,
                attributes: vec![],
                name: "ValuesUnion".into(),
                variants: vec![
                    RSEnumVariant {
                        doc: None,
                        name: "Boolean".into(),
                        attributes: vec![],
                        descriptor: RSEnumVariantDescriptor::Descriptor(
                            RSDescriptor::Primitive(RSPrimitive::Boolean).into()
                        ),
                    },
                    RSEnumVariant {
                        doc: None,
                        name: "String".into(),
                        attributes: vec![],
                        descriptor: RSEnumVariantDescriptor::Descriptor(
                            RSDescriptor::Primitive(RSPrimitive::String).into()
                        ),
                    }
                ],
            })
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            r#"pub enum ValuesUnion {
    Boolean(bool),
    String(String),
}"#
        );
    }
}
