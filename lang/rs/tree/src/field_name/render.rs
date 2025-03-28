use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;
use genotype_lang_rs_core::RSNaming;
use miette::Result;

use crate::RSRender;

use super::RSFieldName;

impl RSRender for RSFieldName {
    fn render(&self, _indent: &GTIndent, _config: &RSLangConfig) -> Result<String> {
        Ok(RSNaming::render(&self.0))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indent::rs_indent;

    #[test]
    fn test_render() {
        assert_eq!(
            RSFieldName("foo".into())
                .render(&rs_indent(), &Default::default())
                .unwrap(),
            "foo"
        );
    }

    #[test]
    fn test_render_keyword() {
        assert_eq!(
            RSFieldName("type".into())
                .render(&rs_indent(), &Default::default())
                .unwrap(),
            "r#type"
        );
    }
}
