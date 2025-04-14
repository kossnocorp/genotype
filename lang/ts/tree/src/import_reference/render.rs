use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for TSImportReference {
    type RenderState = TSRenderState;

    type RenderContext = TSRenderContext;

    fn render(&self, state: Self::RenderState, context: &mut Self::RenderContext) -> Result<String> {
        Ok(match self {
            TSImportReference::Default(name) => name.clone(),

            TSImportReference::Glob(name) => format!("* as {}", name),

            TSImportReference::Named(names) => {
                let names = names
                    .iter()
                    .map(|name| name.render(state, context))
                    .collect::<Result<Vec<_>>>()?
                    .join(", ");
                format!("{{ {} }}", names)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_default() {
        assert_eq!(
            TSImportReference::Default("Name".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            "Name"
        );
    }

    #[test]
    fn test_render_glob() {
        assert_eq!(
            TSImportReference::Glob("name".into())
                .render(Default::default(), &mut Default::default())
                .unwrap(),
            "* as name"
        );
    }

    #[test]
    fn test_render_named() {
        assert_eq!(
            TSImportReference::Named(vec![
                TSImportName::Name("Name".into()),
                TSImportName::Alias("Name".into(), "Alias".into()),
            ])
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            "{ Name, Name as Alias }"
        );
    }
}
