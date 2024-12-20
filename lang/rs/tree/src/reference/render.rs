use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;
use miette::Result;

use crate::RSRender;

use super::RSReference;

impl RSRender for RSReference {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> Result<String> {
        self.identifier.render(indent, config)
    }
}

#[cfg(test)]
mod tests {
    use genotype_parser::{GTDefinitionId, GTReferenceId};

    use super::*;
    use crate::indent::rs_indent;

    #[test]
    fn test_render() {
        assert_eq!(
            "Foo",
            RSReference {
                id: GTReferenceId("module".into(), (0, 0).into()),
                identifier: "Foo".into(),
                definition_id: GTDefinitionId("module".into(), "Foo".into())
            }
            .render(&rs_indent(), &Default::default())
            .unwrap(),
        );
    }
}
