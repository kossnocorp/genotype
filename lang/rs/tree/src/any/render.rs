use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;
use miette::Result;

use crate::RSRender;

use super::RSAny;

impl RSRender for RSAny {
    fn render(&self, _indent: &GTIndent, config: &RSLangConfig) -> Result<String> {
        Ok("Value".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indent::rs_indent;

    #[test]
    fn test_render_primitive() {
        assert_eq!(
            RSAny.render(&rs_indent(), &Default::default()).unwrap(),
            "Value"
        );
    }
}
