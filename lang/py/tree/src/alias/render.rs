use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};
use genotype_lang_py_config::PYLangConfig;
use genotype_lang_py_config::PYVersion;

use crate::PYRender;

use super::PYAlias;

impl PYRender for PYAlias {
    fn render(&self, indent: &GTIndent, config: &PYLangConfig) -> String {
        let name = self.name.render(indent);
        let descriptor = self.descriptor.render(indent, config);

        if let PYVersion::Legacy = config.version {
            format!("{} : TypeAlias = {}", name, descriptor)
        } else {
            format!("type {} = {}", name, descriptor)
        }
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_py_config::PYLangConfig;
    use genotype_lang_py_config::PYVersion;
    use pretty_assertions::assert_eq;

    use crate::*;

    #[test]
    fn test_render() {
        assert_eq!(
            PYAlias {
                name: "Name".into(),
                descriptor: PYDescriptor::Primitive(PYPrimitive::String)
            }
            .render(&py_indent(), &Default::default()),
            "type Name = str"
        );
    }

    #[test]
    fn test_render_legacy() {
        assert_eq!(
            PYAlias {
                name: "Name".into(),
                descriptor: PYDescriptor::Primitive(PYPrimitive::String)
            }
            .render(&py_indent(), &PYLangConfig::new(PYVersion::Legacy)),
            "Name : TypeAlias = str"
        );
    }
}
