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
    
    use crate::test::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_alias() {
        assert_snapshot!(
            render_node(Tst::definition(Tst::alias("Name", Tst::primitive_string()))),
            @"export type Name = string;"
        );
    }

    #[test]
    fn test_render_interface() {
        assert_snapshot!(
            render_node(
                Tst::definition(Tst::interface(
                    "Name",
                    vec![
                        Tst::property("name", Tst::primitive_string()),
                        Tst::property_optional("age", Tst::primitive_number()),
                    ],
                )),
            ),
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
            render_node(
                Tst::definition(Tst::branded("Version", Tst::primitive_number())),
            ),
            @"
        export type Version = number & { [versionBrand]: true };
        declare const versionBrand: unique symbol;
        "
        );
    }

    #[test]
    fn test_render_embed() {
        assert_snapshot!(
            render_node(
                Tst::definition(Tst::embed_definition(
                    "Name",
                    r#"const hello = {
  name: "World"
};"#,
                )),
            ),
            @r#"
        const hello = {
          name: "World"
        };
        "#
        );
    }
}
