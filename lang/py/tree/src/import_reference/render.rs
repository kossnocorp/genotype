use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for PYImportReference {
    type RenderContext = PYRenderContext<'a>;

    fn render(&self, context: &mut Self::RenderContext) -> Result<String> {
        Ok(match self {
            PYImportReference::Default(name) => {
                if let Some(name) = name {
                    name.render(context)?
                } else {
                    "".into()
                }
            }

            PYImportReference::Glob => "*".into(),

            PYImportReference::Named(names) => names
                .iter()
                .map(|name| name.render(context))
                .collect::<Result<Vec<_>>>()?
                .join(", "),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_default() {
        assert_eq!(
            PYImportReference::Default(Some("Name".into()))
                .render(&mut Default::default())
                .unwrap(),
            "Name"
        );
        assert_eq!(
            PYImportReference::Default(None)
                .render(&mut Default::default())
                .unwrap(),
            ""
        );
    }

    #[test]
    fn test_render_glob() {
        assert_eq!(
            PYImportReference::Glob
                .render(&mut Default::default())
                .unwrap(),
            "*"
        );
    }

    #[test]
    fn test_render_named() {
        assert_eq!(
            PYImportReference::Named(vec![
                PYImportName::Name("Name".into()),
                PYImportName::Alias("Name".into(), "Alias".into()),
            ])
            .render(&mut Default::default())
            .unwrap(),
            "Name, Name as Alias"
        );
    }
}
