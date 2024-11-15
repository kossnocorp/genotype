use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;
use miette::Result;

use crate::RSRender;

use super::RSImportName;

impl RSRender for RSImportName {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> Result<String> {
        Ok(match self {
            RSImportName::Name(name) => name.render(indent, config)?,
            RSImportName::Alias(name, alias) => {
                format!(
                    "{name} as {alias}",
                    name = name.render(indent, config)?,
                    alias = alias.render(indent, config)?
                )
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indent::rs_indent;

    #[test]
    fn test_render_name() {
        assert_eq!(
            RSImportName::Name("Name".into())
                .render(&rs_indent(), &Default::default())
                .unwrap(),
            "Name"
        );
    }

    #[test]
    fn test_render_alias() {
        assert_eq!(
            RSImportName::Alias("Name".into(), "Alias".into())
                .render(&rs_indent(), &Default::default())
                .unwrap(),
            "Name as Alias"
        );
    }
}
