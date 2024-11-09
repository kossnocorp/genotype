use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_core_tree::render::GTRender;
use genotype_lang_rs_config::RSLangConfig;
use genotype_lang_rs_config::RSVersion;

use crate::RSRender;

use super::RSDict;

impl RSRender for RSDict {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> String {
        format!(
            "{}{}{}, {}{}",
            if let RSVersion::Legacy = config.version {
                "Dict"
            } else {
                "dict"
            },
            "[",
            self.key.render(indent),
            self.descriptor.render(indent, config),
            "]"
        )
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_rs_config::RSLangConfig;
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::*;

    #[test]
    fn test_render() {
        assert_eq!(
            RSDict {
                key: RSDictKey::String,
                descriptor: RSDescriptor::Primitive(RSPrimitive::Int),
            }
            .render(&rs_indent(), &Default::default()),
            "dict[str, isize]"
        );
    }

    #[test]
    fn test_render_legacy() {
        assert_eq!(
            RSDict {
                key: RSDictKey::String,
                descriptor: RSDescriptor::Primitive(RSPrimitive::Int),
            }
            .render(&rs_indent(), &RSLangConfig::new(RSVersion::Legacy)),
            "Dict[str, isize]"
        );
    }
}
