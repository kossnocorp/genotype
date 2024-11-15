use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;
use miette::Result;

use crate::RSRender;

use super::RSImportReference;

impl RSRender for RSImportReference {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> Result<String> {
        Ok(match self {
            RSImportReference::Module => "".into(),

            RSImportReference::Glob => "*".into(),

            RSImportReference::Named(names) => {
                let names = names
                    .iter()
                    .map(|name| name.render(indent, config))
                    .collect::<Result<Vec<String>>>()?
                    .join(", ");
                format!("{{{}}}", names)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_render_module() {
        assert_eq!(
            RSImportReference::Module
                .render(&rs_indent(), &Default::default())
                .unwrap(),
            ""
        );
    }

    #[test]
    fn test_render_glob() {
        assert_eq!(
            RSImportReference::Glob
                .render(&rs_indent(), &Default::default())
                .unwrap(),
            "*"
        );
    }

    #[test]
    fn test_render_named() {
        assert_eq!(
            RSImportReference::Named(vec![
                RSImportName::Name("Name".into()),
                RSImportName::Alias("Name".into(), "Alias".into()),
            ])
            .render(&rs_indent(), &Default::default())
            .unwrap(),
            "{Name, Name as Alias}"
        );
    }
}
