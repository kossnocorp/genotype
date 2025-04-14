use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for PYImportName {
    type RenderState = PYRenderState;

    type RenderContext = PYRenderContext<'a>;

    fn render(&self, state: Self::RenderState, context: &mut Self::RenderContext) -> Result<String> {
        match self {
            PYImportName::Name(name) => name.render(state, context),

            PYImportName::Alias(name, alias) => {
                let name = name.render(state, context)?;
                let alias = alias.render(state, context)?;
                Ok(format!("{name} as {alias}"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_name() {
        assert_eq!(
            PYImportName::Name("Name".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            "Name"
        );
    }

    #[test]
    fn test_render_alias() {
        assert_eq!(
            PYImportName::Alias("Name".into(), "Alias".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            "Name as Alias"
        );
    }
}
