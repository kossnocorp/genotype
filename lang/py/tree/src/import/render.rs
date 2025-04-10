use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for PYImport {
    type RenderContext = PYRenderContext<'a>;

    fn render(&self, context: &mut Self::RenderContext) -> Result<String> {
        let path = self.path.render(context)?;
        let reference = self.reference.render(context)?;

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
    use pretty_assertions::assert_eq;

    #[test]
    fn test_render_default() {
        assert_eq!(
            PYImport {
                path: ".path.to.module".into(),
                reference: PYImportReference::Default(Some("name".into())),
                dependency: PYDependency::Local(".path.to.module".into())
            }
            .render(&mut Default::default())
            .unwrap(),
            r#"import .path.to.module as name"#
        );
        assert_eq!(
            PYImport {
                path: ".path.to.module".into(),
                reference: PYImportReference::Default(None),
                dependency: PYDependency::Local(".path.to.module".into())
            }
            .render(&mut Default::default())
            .unwrap(),
            r#"import .path.to.module"#
        );
    }

    #[test]
    fn test_render_glob() {
        assert_eq!(
            PYImport {
                path: ".path.to.module".into(),
                reference: PYImportReference::Glob,
                dependency: PYDependency::Local(".path.to.module".into())
            }
            .render(&mut Default::default())
            .unwrap(),
            r#"from .path.to.module import *"#
        );
    }

    #[test]
    fn test_render_named() {
        assert_eq!(
            PYImport {
                path: ".path.to.module".into(),
                reference: PYImportReference::Named(vec![
                    PYImportName::Name("Name".into()),
                    PYImportName::Alias("Name".into(), "Alias".into()),
                ]),
                dependency: PYDependency::Local(".path.to.module".into())
            }
            .render(&mut Default::default())
            .unwrap(),
            r#"from .path.to.module import Name, Name as Alias"#
        );
    }
}
