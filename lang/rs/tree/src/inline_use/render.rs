use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;
use miette::Result;

use crate::RSRender;

use super::RSInlineUse;

impl RSRender for RSInlineUse {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> Result<String> {
        let module = self.path.render(indent, config)?;
        let name = self.name.render(indent, config)?;
        Ok(format!("{module}::{name}"))
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::*;

    #[test]
    fn test_render() {
        assert_eq!(
            RSInlineUse {
                path: "self::path::to::module".into(),
                name: "Name".into(),
            }
            .render(&rs_indent(), &Default::default())
            .unwrap(),
            "self::path::to::module::Name"
        );
    }
}
