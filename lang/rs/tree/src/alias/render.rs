use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};
use genotype_lang_rs_config::RSLangConfig;
use genotype_lang_rs_config::RSVersion;

use crate::RSRender;

use super::RSAlias;

impl RSRender for RSAlias {
    fn render(&self, indent: &GTIndent, config: &RSLangConfig) -> String {
        let name = self.name.render(indent);
        let descriptor = self.descriptor.render(indent, config);
        let r#type = format!("type {} = {};", name, descriptor);

        if let Some(doc) = &self.doc {
            format!("{}\n{}", doc.render(&indent), r#type)
        } else {
            r#type
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
            RSAlias {
                doc: None,
                name: "Name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
            }
            .render(&rs_indent(), &Default::default()),
            "type Name = String;"
        );
    }

    #[test]
    fn test_render_doc() {
        assert_eq!(
            RSAlias {
                doc: Some("Hello, world!".into()),
                name: "Name".into(),
                descriptor: RSDescriptor::Primitive(RSPrimitive::String),
            }
            .render(&rs_indent(), &Default::default()),
            r#"/// Hello, world!
type Name = String;"#
        );
    }
}
