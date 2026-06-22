use crate::prelude::internal::*;

impl<'context> GtlRender<'context, TsRenderTypes> for TsImportReference {
    fn render(
        &self,
        state: TsRenderState,
        context: &mut TsRenderContext,
    ) -> TsRenderResult<String> {
        Ok(match self {
            TsImportReference::Default(name) => name.to_string(),

            TsImportReference::Glob(name) => format!("* as {}", name),

            TsImportReference::Named(names) => {
                let names = names
                    .iter()
                    .map(|name| name.render(state, context))
                    .collect::<Result<Vec<_>, _>>()?
                    .join(", ");
                format!("{{ {} }}", names)
            }
        })
    }
}

#[cfg(test)]
mod tests {

    use crate::test::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_default() {
        assert_snapshot!(
            render_node(Tst::import_reference_default("Name")),
            @"Name"
        );
    }

    #[test]
    fn test_render_glob() {
        assert_snapshot!(
            render_node(Tst::import_reference_glob("name")),
            @"* as name"
        );
    }

    #[test]
    fn test_render_named() {
        assert_snapshot!(
            render_node(
                Tst::import_reference_named(vec![
                    Tst::import_name("Name"),
                    Tst::import_alias("Name", "Alias"),
                ]),
            ),
            @"{ Name, Name as Alias }"
        );
    }
}
