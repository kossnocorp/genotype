use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;
use miette::Result;

use crate::RSRender;

use super::RSHashMap;

impl RSRender for RSHashMap {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> Result<String> {
        let key = self.key.render(indent, config)?;
        let descriptor = self.descriptor.render(indent, config)?;
        Ok(format!("HashMap<{key}, {descriptor}>"))
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
            .render(&rs_indent(), &Default::default())
            .unwrap(),
            "HashMap<String, isize>"
        );
    }
}
