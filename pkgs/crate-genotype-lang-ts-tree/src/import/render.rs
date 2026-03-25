use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for TsImport {
    type RenderState = TsRenderState;

    type RenderContext = TsRenderContext<'a>;

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
    use insta::assert_snapshot;

    #[test]
    fn test_render_default() {
        assert_snapshot!(
            TsImport {
                path: "../path/to/module".into(),
                reference: TsImportReference::Default("Name".into()),
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @r#"import Name from "../path/to/module.js";"#
        );
    }

    #[test]
    fn test_render_glob() {
        assert_snapshot!(
            TsImport {
                path: "../path/to/module".into(),
                reference: TsImportReference::Glob("name".into()),
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @r#"import * as name from "../path/to/module.js";"#
        );
    }

    #[test]
    fn test_render_named() {
        assert_snapshot!(
            TsImport {
                path: "../path/to/module".into(),
                reference: TsImportReference::Named(vec![
                    TsImportName::Name("Name".into()),
                    TsImportName::Alias("Name".into(), "Alias".into()),
                ])
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @r#"import { Name, Name as Alias } from "../path/to/module.js";"#
        );
    }
}
