use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsImportName {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        match self {
            TsImportName::Name(name) => name.render(state, context),

            TsImportName::Alias(name, alias) => {
                let name = name.render(state, context)?;
                let alias = alias.render(state, context)?;

                Ok(format!("{name} as {alias}"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    
    use crate::test::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_name() {
        assert_snapshot!(
            render_node(Tst::import_name("Name")),
            @"Name"
        );
    }

    #[test]
    fn test_render_alias() {
        assert_snapshot!(
            render_node(Tst::import_alias("Name", "Alias")),
            @"Name as Alias"
        );
    }
}
