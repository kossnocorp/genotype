use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;
use miette::Result;

use crate::RSRender;

use super::RSPath;

impl RSRender for RSPath {
    fn render(&self, _indent: &GTIndent, _config: &RSLangConfig) -> Result<String> {
        Ok(self.1.clone())
    }
}

#[cfg(test)]
mod tests {
    use genotype_parser::GTModuleId;

    use super::*;
    use crate::indent::rs_indent;

    #[test]
    fn test_render() {
        assert_eq!(
            RSPath(
                GTModuleId("path/to/module".into()),
                "self::path::to::module".into()
            )
            .render(&rs_indent(), &Default::default())
            .unwrap(),
            "self::path::to::module"
        );
    }
}
