use genotype_config::GTConfig;
use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_py_config::PYVersion;

use crate::PYRender;

use super::PYList;

impl PYRender for PYList {
    fn render(&self, indent: &GTIndent, config: &GTConfig) -> String {
        format!(
            "{}[{}]",
            if let PYVersion::Legacy = config.python_version() {
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
    use genotype_lang_py_config::PYConfig;

    use crate::{descriptor::PYDescriptor, indent::py_indent, primitive::PYPrimitive};

    use super::*;

    #[test]
    fn test_render_array() {
        assert_eq!(
            PYList {
                descriptor: PYDescriptor::Primitive(PYPrimitive::String)
            }
            .render(&py_indent(), &Default::default()),
            "list[str]"
        );
    }

    #[test]
    fn test_render_legacy() {
        assert_eq!(
            PYList {
                descriptor: PYDescriptor::Primitive(PYPrimitive::String)
            }
            .render(
                &py_indent(),
                &GTConfig::default().with_python(PYConfig::new(PYVersion::Legacy))
            ),
            "List[str]"
        );
    }
}
