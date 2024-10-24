use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::TSDoc;

impl TSDoc {
    pub fn with_doc(doc: &Option<TSDoc>, indent: &GTIndent, str: String, padded: bool) -> String {
        format!(
            "{}{}",
            if let Some(doc) = doc {
                doc.render(indent) + if padded { "\n\n" } else { "\n" }
            } else {
                String::new()
            },
            str
        )
    }
}

impl GTRender for TSDoc {
    fn render(&self, indent: &GTIndent) -> String {
        let lines = self.0.split("\n").enumerate();
        lines
            .map(|(index, line)| {
                format!(
                    "{}{} {}",
                    indent.string,
                    if index == 0 { "/**" } else { " *" },
                    line
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
            + " */"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indent::ts_indent;

    #[test]
    fn test_render_simple() {
        assert_eq!(
            TSDoc("Hello, world!".into()).render(&ts_indent()),
            "/** Hello, world! */"
        );
    }

    #[test]
    fn test_render_multiline() {
        assert_eq!(
            TSDoc(
                r#"Hello,
cruel
world!"#
                    .into()
            )
            .render(&ts_indent()),
            r#"/** Hello,
 * cruel
 * world! */"#
        );
    }

    #[test]
    fn test_render_indent() {
        assert_eq!(
            TSDoc(
                r#"Hello,
cruel
world!"#
                    .into()
            )
            .render(&ts_indent().increment()),
            r#"  /** Hello,
   * cruel
   * world! */"#
        );
    }

    #[test]
    fn test_with_doc_some() {
        assert_eq!(
            TSDoc::with_doc(
                &Some(TSDoc("Hello, world!".into())),
                &ts_indent(),
                "type Name = string;".into(),
                false
            ),
            r#"/** Hello, world! */
type Name = string;"#
        );
    }

    #[test]
    fn test_with_doc_none() {
        assert_eq!(
            TSDoc::with_doc(&None, &ts_indent(), "type Name = string;".into(), false),
            r#"type Name = string;"#
        );
    }

    #[test]
    fn test_with_doc_padded_some() {
        assert_eq!(
            TSDoc::with_doc(
                &Some(TSDoc("Hello, world!".into())),
                &ts_indent(),
                "type Name = string;".into(),
                true
            ),
            r#"/** Hello, world! */

type Name = string;"#
        );
    }

    #[test]
    fn test_with_doc_padded_none() {
        assert_eq!(
            TSDoc::with_doc(&None, &ts_indent(), "type Name = string;".into(), true),
            r#"type Name = string;"#
        );
    }
}
