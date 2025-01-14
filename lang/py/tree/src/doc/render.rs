use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use super::PYDoc;

impl GTRender for PYDoc {
    fn render(&self, indent: &GTIndent) -> String {
        let lines = self.0.split("\n").enumerate();
        lines
            .map(|(index, line)| {
                format!(
                    "{}{}{}",
                    indent.string,
                    if index == 0 { r#"""""# } else { "" },
                    line
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
            + r#"""""#
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use super::*;
    use crate::indent::py_indent;

    #[test]
    fn test_render_simple() {
        assert_eq!(
            PYDoc("Hello, world!".into()).render(&py_indent()),
            r#""""Hello, world!""""#
        );
    }

    #[test]
    fn test_render_multiline() {
        assert_eq!(
            PYDoc(
                r#"Hello,
cruel
world!"#
                    .into()
            )
            .render(&py_indent()),
            r#""""Hello,
cruel
world!""""#
        );
    }

    #[test]
    fn test_render_indent() {
        assert_eq!(
            PYDoc(
                r#"Hello,
cruel
world!"#
                    .into()
            )
            .render(&py_indent().increment()),
            r#"    """Hello,
    cruel
    world!""""#
        );
    }
}
