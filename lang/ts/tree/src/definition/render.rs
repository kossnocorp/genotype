use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TSDefinition {
    type RenderState = TSRenderState;

    type RenderContext = TSRenderContext;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        match self {
            TSDefinition::Alias(alias) => alias.render(state, context),
            TSDefinition::Interface(interface) => interface.render(state, context),
            TSDefinition::Branded(branded) => branded.render(state, context),
            TSDefinition::Embed(embed) => embed.render(state, context),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render_alias() {
        assert_eq!(
            TSDefinition::Alias(TSAlias {
                doc: None,
                name: "Name".into(),
                descriptor: TSDescriptor::Primitive(TSPrimitive::String),
            })
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            "export type Name = string;"
        );
    }

    #[test]
    fn test_render_interface() {
        assert_eq!(
            TSDefinition::Interface(TSInterface {
                doc: None,
                name: "Name".into(),
                extensions: vec![],
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
            r#"export interface Name {
  name: string;
  age?: number;
}"#
        );
    }

    #[test]
    fn test_render_branded() {
        assert_eq!(
            TSDefinition::Branded(TSBranded {
                doc: None,
                name: "Version".into(),
                primitive: TSPrimitive::Number
            })
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            r#"export type Version = number & { [versionBrand]: true };
declare const versionBrand: unique symbol;"#
        );
    }

    #[test]
    fn test_render_embed() {
        assert_eq!(
            TSDefinition::Embed(TSEmbedDefinition {
                name: "Name".into(),
                embed: r#"const hello = {
  name: "World"
};"#
                .into()
            })
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            r#"const hello = {
  name: "World"
};"#
        );
    }
}
