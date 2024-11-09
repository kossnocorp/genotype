use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::RSDoc;

impl GTRender for RSDoc {
    fn render(&self, indent: &GTIndent) -> String {
        self.0
            .split("\n")
            .map(|line| {
                format!(
                    r#"{}{} {}"#,
                    indent.string,
                    if self.1 { "//!" } else { "///" },
                    line
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::indent::rs_indent;

    #[test]
    fn test_render_simple() {
        assert_eq!(
            RSDoc::new("Hello, world!", false).render(&rs_indent()),
            r#"/// Hello, world!"#
        );
    }

    #[test]
    fn test_render_module() {
        assert_eq!(
            RSDoc::new("Hello, world!", true).render(&rs_indent()),
            r#"//! Hello, world!"#
        );
    }

    #[test]
    fn test_render_multiline() {
        assert_eq!(
            RSDoc::new(
                r#"Hello,
cruel
world!"#,
                false
            )
            .render(&rs_indent()),
            r#"/// Hello,
/// cruel
/// world!"#
        );
    }

    #[test]
    fn test_render_indent() {
        assert_eq!(
            RSDoc::new(
                r#"Hello,
cruel
world!"#,
                false
            )
            .render(&rs_indent().increment()),
            r#"    /// Hello,
    /// cruel
    /// world!"#
        );
    }
}
