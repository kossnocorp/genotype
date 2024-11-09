use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;

use crate::RSRender;

use super::RSHashMap;

impl RSRender for RSHashMap {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> String {
        format!(
            "HashMap<{}, {}>",
            self.key.render(indent, config),
            self.descriptor.render(indent, config),
        )
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::*;

    #[test]
    fn test_render() {
        assert_eq!(
            RSHashMap {
                key: RSPrimitive::String.into(),
                descriptor: RSPrimitive::Int.into(),
            }
            .render(&rs_indent(), &Default::default()),
            "HashMap<String, isize>"
        );
    }
}
