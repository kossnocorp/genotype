use crate::prelude::internal::*;

impl<'context> GtlRender<'context, PyRenderTypes> for PyImport {
    fn render(
        &self,
        state: PyRenderState,
        context: &mut PyRenderContext,
    ) -> PyRenderResult<String> {
        let path = self.dependency.as_path().render(state, context)?;
        let reference = self.reference.render(state, context)?;

        Ok(match self.reference {
            PyImportReference::Default(_) => {
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
            PyImport {
                reference: PyImportReference::Default(Some("name".into())),
                dependency: PyDependencyIdent::Local(".path.to.module".into())
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"import .path.to.module as name"
        );
        assert_snapshot!(
            PyImport {
                reference: PyImportReference::Default(None),
                dependency: PyDependencyIdent::Local(".path.to.module".into())
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"import .path.to.module"
        );
    }

    #[test]
    fn test_render_glob() {
        assert_snapshot!(
            PyImport {
                reference: PyImportReference::Glob,
                dependency: PyDependencyIdent::Local(".path.to.module".into())
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"from .path.to.module import *"
        );
    }

    #[test]
    fn test_render_named() {
        assert_snapshot!(
            PyImport {
                reference: PyImportReference::Named(vec![
                    PyImportName::Name("Name".into()),
                    PyImportName::Alias("Name".into(), "Alias".into()),
                ]),
                dependency: PyDependencyIdent::Local(".path.to.module".into())
            }
            .render(Default::default(), &mut Default::default())
            .unwrap(),
            @"from .path.to.module import Name, Name as Alias"
        );
    }
}
