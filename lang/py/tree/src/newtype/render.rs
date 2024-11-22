use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};
use genotype_lang_py_config::PYLangConfig;

use crate::PYRender;

use super::PYNewtype;

impl PYRender for PYNewtype {
    fn render(&self, indent: &GTIndent, _config: &PYLangConfig) -> String {
        let mut blocks = vec![];

        let name = self.name.render(indent);
        let primitive = self.primitive.render(indent);
        blocks.push(format!(r#"{name} = NewType("{name}", {primitive})"#));

        if let Some(doc) = &self.doc {
            blocks.push(doc.render(&indent));
        }

        blocks.join("\n")
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::*;

    #[test]
    fn test_render() {
        assert_eq!(
            PYNewtype {
                doc: None,
                name: "UserId".into(),
                primitive: PYPrimitive::String,
            }
            .render(&py_indent(), &Default::default()),
            r#"UserId = NewType("UserId", str)"#
        );
    }

    #[test]
    fn test_render_doc() {
        assert_eq!(
            PYNewtype {
                doc: Some(PYDoc("Hello, world!".into())),
                name: "UserId".into(),
                primitive: PYPrimitive::String,
            }
            .render(&py_indent(), &Default::default()),
            r#"UserId = NewType("UserId", str)
"""Hello, world!""""#
        );
    }
}
