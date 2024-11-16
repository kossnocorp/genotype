use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;
use miette::Result;

use crate::RSRender;

use super::RSUseReference;

impl RSRender for RSUseReference {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> Result<String> {
        Ok(match self {
            RSUseReference::Module => "".into(),

            RSUseReference::Glob => "*".into(),

            RSUseReference::Named(names) => {
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
    use crate::*;

    #[test]
    fn test_render_module() {
        assert_eq!(
            RSUseReference::Module
                .render(&rs_indent(), &Default::default())
                .unwrap(),
            ""
        );
    }

    #[test]
    fn test_render_glob() {
        assert_eq!(
            RSUseReference::Glob
                .render(&rs_indent(), &Default::default())
                .unwrap(),
            "*"
        );
    }

    #[test]
    fn test_render_named() {
        assert_eq!(
            RSUseReference::Named(vec![
                RSUseName::Name("Name".into()),
                RSUseName::Alias("Name".into(), "Alias".into()),
            ])
            .render(&rs_indent(), &Default::default())
            .unwrap(),
            "{Name, Name as Alias}"
        );
    }
}
