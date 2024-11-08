use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;
use genotype_lang_rs_config::RSVersion;

use crate::RSRender;

use super::RSList;

impl RSRender for RSList {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> String {
        format!(
            "{}[{}]",
            if let RSVersion::Legacy = config.version {
                "List"
            } else {
                "list"
            },
            self.descriptor.render(indent, config)
        )
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_rs_config::RSLangConfig;

    use crate::{descriptor::RSDescriptor, indent::rs_indent, primitive::RSPrimitive};

    use super::*;

    #[test]
    fn test_render_array() {
        assert_eq!(
            RSList {
                descriptor: RSDescriptor::Primitive(RSPrimitive::String)
            }
            .render(&rs_indent(), &Default::default()),
            "list[str]"
        );
    }

    #[test]
    fn test_render_legacy() {
        assert_eq!(
            RSList {
                descriptor: RSDescriptor::Primitive(RSPrimitive::String)
            }
            .render(&rs_indent(), &RSLangConfig::new(RSVersion::Legacy)),
            "List[str]"
        );
    }
}
