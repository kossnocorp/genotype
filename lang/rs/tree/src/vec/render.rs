use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;
use miette::Result;

use crate::RSRender;

use super::RSVec;

impl RSRender for RSVec {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> Result<String> {
        let descriptor = self.descriptor.render(indent, config)?;
        Ok(format!("Vec<{descriptor}>"))
    }
}

#[cfg(test)]
mod tests {
    use crate::{descriptor::RSDescriptor, indent::rs_indent, primitive::RSPrimitive};

    use super::*;

    #[test]
    fn test_render_array() {
        assert_eq!(
            RSVec {
                descriptor: RSDescriptor::Primitive(RSPrimitive::String)
            }
            .render(&rs_indent(), &Default::default())
            .unwrap(),
            "Vec<String>"
        );
    }
}
