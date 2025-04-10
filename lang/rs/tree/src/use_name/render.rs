use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for RSUseName {
    type RenderContext = RSRenderContext<'a>;

    fn render(&self, context: &mut Self::RenderContext) -> Result<String> {
        Ok(match self {
            RSUseName::Name(name) => name.render(context)?,
            RSUseName::Alias(name, alias) => {
                format!(
                    "{name} as {alias}",
                    name = name.render(context)?,
                    alias = alias.render(context)?
                )
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_name() {
        assert_eq!(
            RSUseName::Name("Name".into())
                .render(&mut Default::default())
                .unwrap(),
            "Name"
        );
    }

    #[test]
    fn test_render_alias() {
        assert_eq!(
            RSUseName::Alias("Name".into(), "Alias".into())
                .render(&mut Default::default())
                .unwrap(),
            "Name as Alias"
        );
    }
}
