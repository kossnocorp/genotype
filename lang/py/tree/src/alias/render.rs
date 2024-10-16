use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use crate::{PYOptions, PYRender, PYVersion};

use super::PYAlias;

impl PYRender for PYAlias {
    fn render(&self, indent: &GTIndent, options: &PYOptions) -> String {
        let name = self.name.render(indent);
        let descriptor = self.descriptor.render(indent, options);

        if let PYVersion::Legacy = options.version {
            format!("{} : TypeAlias = {}", name, descriptor)
        } else {
            format!("type {} = {}", name, descriptor)
        }
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::*;

    #[test]
    fn test_render() {
        assert_eq!(
            PYAlias {
                name: "Name".into(),
                descriptor: PYDescriptor::Primitive(PYPrimitive::String)
            }
            .render(&py_indent(), &PYOptions::default()),
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
            .render(&py_indent(), &PYOptions::new(PYVersion::Legacy)),
            "Name : TypeAlias = str"
        );
    }
}
