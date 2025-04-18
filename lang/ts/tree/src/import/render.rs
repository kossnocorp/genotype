use crate::*;
use genotype_lang_core_tree::*;
use miette::Result;

impl<'a> GtlRender<'a> for TSImport {
    type RenderContext = TSRenderContext<'a>;

    fn render(&self, context: &mut Self::RenderContext) -> Result<String> {
        let reference = self.reference.render(context)?;
        let path = self.path.render(context)?;

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
                path: "../path/to/module.ts".into(),
                reference: TSImportReference::Default("Name".into()),
            }
            .render(&mut Default::default())
            .unwrap(),
            r#"import Name from "../path/to/module.ts";"#
        );
    }

    #[test]
    fn test_render_glob() {
        assert_eq!(
            TSImport {
                path: "../path/to/module.ts".into(),
                reference: TSImportReference::Glob("name".into()),
            }
            .render(&mut Default::default())
            .unwrap(),
            r#"import * as name from "../path/to/module.ts";"#
        );
    }

    #[test]
    fn test_render_named() {
        assert_eq!(
            TSImport {
                path: "../path/to/module.ts".into(),
                reference: TSImportReference::Named(vec![
                    TSImportName::Name("Name".into()),
                    TSImportName::Alias("Name".into(), "Alias".into()),
                ])
            }
            .render(&mut Default::default())
            .unwrap(),
            r#"import { Name, Name as Alias } from "../path/to/module.ts";"#
        );
    }
}
