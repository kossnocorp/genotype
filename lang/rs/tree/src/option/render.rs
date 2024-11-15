use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;
use miette::Result;

use crate::RSRender;

use super::RSOption;

impl RSRender for RSOption {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> Result<String> {
        let descriptor = self.descriptor.render(indent, config)?;
        Ok(format!("Option<{descriptor}>"))
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_render() {
        assert_eq!(
            RSOption {
                descriptor: RSDescriptor::Primitive(RSPrimitive::String)
            }
            .render(&rs_indent(), &Default::default())
            .unwrap(),
            "Option<String>"
        );
    }
}
