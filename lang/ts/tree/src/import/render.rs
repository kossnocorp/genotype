use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TSImport {
    type RenderState = TSRenderState;

    type RenderContext = TSRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let reference = self.reference.render(state, context)?;
        let path = self.path.render(state, context)?;

        Ok(format!(r#"import {reference} from "{path}";"#))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render_default() {
        assert_eq!(
            TSImport {
                path: "../path/to/module".into(),
                reference: TSImportReference::Default("Name".into()),
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            r#"import Name from "../path/to/module.js";"#
        );
    }

    #[test]
    fn test_render_glob() {
        assert_eq!(
            TSImport {
                path: "../path/to/module".into(),
                reference: TSImportReference::Glob("name".into()),
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            r#"import * as name from "../path/to/module.js";"#
        );
    }

    #[test]
    fn test_render_named() {
        assert_eq!(
            TSImport {
                path: "../path/to/module".into(),
                reference: TSImportReference::Named(vec![
                    TSImportName::Name("Name".into()),
                    TSImportName::Alias("Name".into(), "Alias".into()),
                ])
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            r#"import { Name, Name as Alias } from "../path/to/module.js";"#
        );
    }
}
