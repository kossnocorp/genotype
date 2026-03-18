use crate::prelude::internal::*;

impl<'a> GtlRender<'a> for PYImport {
    type RenderState = PYRenderState;

    type RenderContext = PYRenderContext<'a>;

    fn render(
        &self,
        state: Self::RenderState,
        context: &mut Self::RenderContext,
    ) -> Result<String> {
        let path = self.dependency.as_path().render(state, context)?;
        let reference = self.reference.render(state, context)?;

        Ok(match self.reference {
            PYImportReference::Default(_) => {
                if reference.is_empty() {
                    format!(r#"import {}"#, path)
                } else {
                    format!(r#"import {} as {}"#, path, reference)
                }
            }

            _ => {
                format!(r#"from {} import {}"#, path, reference)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_snapshot;

    #[test]
    fn test_render_default() {
        assert_snapshot!(
            PYImport {
                reference: PYImportReference::Default(Some("name".into())),
                dependency: PYDependencyIdent::Path(".path.to.module".into())
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"import .path.to.module as name"
        );
        assert_snapshot!(
            PYImport {
                reference: PYImportReference::Default(None),
                dependency: PYDependencyIdent::Path(".path.to.module".into())
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"import .path.to.module"
        );
    }

    #[test]
    fn test_render_glob() {
        assert_snapshot!(
            PYImport {
                reference: PYImportReference::Glob,
                dependency: PYDependencyIdent::Path(".path.to.module".into())
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"from .path.to.module import *"
        );
    }

    #[test]
    fn test_render_named() {
        assert_snapshot!(
            PYImport {
                reference: PYImportReference::Named(vec![
                    PYImportName::Name("Name".into()),
                    PYImportName::Alias("Name".into(), "Alias".into()),
                ]),
                dependency: PYDependencyIdent::Path(".path.to.module".into())
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"from .path.to.module import Name, Name as Alias"
        );
    }
}
