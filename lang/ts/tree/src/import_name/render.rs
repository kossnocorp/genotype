use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for TSImportName {
    type RenderContext = TSRenderContext<'a>;

    fn render(&self, context: &mut Self::RenderContext) -> Result<String> {
        match self {
            TSImportName::Name(name) => name.render(context),

            TSImportName::Alias(name, alias) => {
                let name = name.render(context)?;
                let alias = alias.render(context)?;

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
            TSImportName::Name("Name".into())
                .render(&mut Default::default())
                .unwrap(),
            "Name"
        );
    }

    #[test]
    fn test_render_alias() {
        assert_eq!(
            TSImportName::Alias("Name".into(), "Alias".into())
                .render(&mut Default::default())
                .unwrap(),
            "Name as Alias"
        );
    }
}
