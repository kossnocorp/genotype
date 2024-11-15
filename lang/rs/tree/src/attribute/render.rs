use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;
use miette::Result;

use crate::RSRender;

use super::RSAttribute;

impl RSRender for RSAttribute {
    fn render(&self, indent: &GTIndent, _config: &RSLangConfig) -> Result<String> {
        Ok(format!(
            "{indent}#[{content}]",
            indent = indent.string,
            content = self.0
        ))
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::*;

    #[test]
    fn test_render() {
        assert_eq!(
            RSAttribute("derive".into())
                .render(&rs_indent(), &Default::default())
                .unwrap(),
            "#[derive]"
        );
    }

    #[test]
    fn test_render_indent() {
        assert_eq!(
            RSAttribute("derive".into())
                .render(&rs_indent().increment(), &Default::default())
                .unwrap(),
            "    #[derive]"
        );
    }
}
