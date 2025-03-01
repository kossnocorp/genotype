use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;
use miette::Result;

use crate::RSRender;

use super::RSFieldName;

impl RSRender for RSFieldName {
    fn render(&self, _indent: &GTIndent, _config: &RSLangConfig) -> Result<String> {
        Ok(self.0.clone())
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
}
