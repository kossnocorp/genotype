use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;
use miette::Result;

use crate::{RSRender, RSUseName};

use super::RSUseReference;

impl RSRender for RSUseReference {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> Result<String> {
        Ok(match self {
            RSUseReference::Module => "".into(),

            RSUseReference::Glob => "*".into(),

            RSUseReference::Named(names) => {
                let names_str = names
                    .iter()
                    .map(|name| name.render(indent, config))
                    .collect::<Result<Vec<String>>>()?
                    .join(", ");
                if names.len() == 1 {
                    if let Some(RSUseName::Name(_)) = names.first() {
                        return Ok(names_str);
                    }
                }

                format!("{{{}}}", names_str)
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

    #[test]
    fn test_render_named_solo() {
        assert_eq!(
            RSUseReference::Named(vec![RSUseName::Name("Name".into()),])
                .render(&rs_indent(), &Default::default())
                .unwrap(),
            "Name"
        );
    }

    #[test]
    fn test_render_named_solo_alias() {
        assert_eq!(
            RSUseReference::Named(vec![RSUseName::Alias("Name".into(), "Alias".into()),])
                .render(&rs_indent(), &Default::default())
                .unwrap(),
            "{Name as Alias}"
        );
    }
}
