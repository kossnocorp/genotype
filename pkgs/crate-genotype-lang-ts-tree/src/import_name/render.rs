use crate::prelude::internal::*;

impl<'context> GtlRender<'context, TsRenderTypes> for TsImportName {
    fn render(
        &self,
        state: TsRenderState,
        context: &mut TsRenderContext,
    ) -> TsRenderResult<String> {
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
