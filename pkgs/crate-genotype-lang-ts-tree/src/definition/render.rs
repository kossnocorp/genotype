use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsDefinition {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        match self {
            TsDefinition::Alias(alias) => alias.render(state, context),
            TsDefinition::Interface(interface) => interface.render(state, context),
            TsDefinition::Branded(branded) => branded.render(state, context),
            TsDefinition::Embed(embed) => embed.render(state, context),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_alias() {
        assert_snapshot!(
            TsDefinition::Alias(TsAlias {
                doc: None,
                name: "Name".into(),
                descriptor: TsDescriptor::Primitive(TsPrimitive::String),
            })
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"export type Name = string;"
        );
    }

    #[test]
    fn test_render_interface() {
        assert_snapshot!(
            TsDefinition::Interface(TsInterface {
                doc: None,
                name: "Name".into(),
                extensions: vec![],
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
        export interface Name {
          name: string;
          age?: number;
        }
        "
        );
    }

    #[test]
    fn test_render_branded() {
        assert_snapshot!(
            TsDefinition::Branded(TsBranded {
                doc: None,
                name: "Version".into(),
                primitive: TsPrimitive::Number
            })
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"
        export type Version = number & { [versionBrand]: true };
        declare const versionBrand: unique symbol;
        "
        );
    }

    #[test]
    fn test_render_embed() {
        assert_snapshot!(
            TsDefinition::Embed(TsEmbedDefinition {
                name: "Name".into(),
                embed: r#"const hello = {
  name: "World"
};"#
                .into()
            })
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @r#"
        const hello = {
          name: "World"
        };
        "#
        );
    }
}
