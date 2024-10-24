use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_core_tree::render::GTRender;
use genotype_lang_py_config::PYLangConfig;
use genotype_lang_py_config::PYVersion;

use crate::PYRender;

use super::PYDict;

impl PYRender for PYDict {
    fn render(&self, indent: &GTIndent, config: &PYLangConfig) -> String {
        format!(
            "{}{}{}, {}{}",
            if let PYVersion::Legacy = config.version {
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
    use genotype_lang_py_config::PYLangConfig;
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::*;

    #[test]
    fn test_render() {
        assert_eq!(
            PYDict {
                key: PYDictKey::String,
                descriptor: PYDescriptor::Primitive(PYPrimitive::Int),
            }
            .render(&py_indent(), &Default::default()),
            "dict[str, int]"
        );
    }

    #[test]
    fn test_render_legacy() {
        assert_eq!(
            PYDict {
                key: PYDictKey::String,
                descriptor: PYDescriptor::Primitive(PYPrimitive::Int),
            }
            .render(&py_indent(), &PYLangConfig::new(PYVersion::Legacy)),
            "Dict[str, int]"
        );
    }
}
