use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use crate::TSDoc;

use super::TSAlias;

impl GTRender for TSAlias {
    fn render(&self, indent: &GTIndent) -> String {
        TSDoc::with_doc(
            &self.doc,
            indent,
            format!(
                "type {} = {};",
                self.name.render(indent),
                self.descriptor.render(indent)
            ),
            false,
        )
    }
}

#[cfg(test)]
mod tests {

    use pretty_assertions::assert_eq;

    use super::*;
    use crate::*;

    #[test]
    fn test_render() {
        assert_eq!(
            TSAlias {
                doc: None,
                name: "Name".into(),
                descriptor: TSDescriptor::Primitive(TSPrimitive::String)
            }
            .render(&ts_indent()),
            "type Name = string;"
        );
    }

    #[test]
    fn test_render_doc() {
        assert_eq!(
            TSAlias {
                doc: Some(TSDoc("Hello, world!".into())),
                name: "Name".into(),
                descriptor: TSDescriptor::Primitive(TSPrimitive::String)
            }
            .render(&ts_indent()),
            r#"/** Hello, world! */
type Name = string;"#
        );
    }
}
