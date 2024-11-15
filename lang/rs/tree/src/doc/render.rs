use genotype_lang_core_tree::indent::GTIndent;
use genotype_lang_rs_config::RSLangConfig;
use miette::Result;

use crate::RSRender;

use super::RSDoc;

impl RSRender for RSDoc {
    fn render(&self, indent: &GTIndent, _config: &RSLangConfig) -> Result<String> {
        Ok(self
            .0
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
            .join("\n"))
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
            RSDoc::new("Hello, world!", false)
                .render(&rs_indent(), &Default::default())
                .unwrap(),
            r#"/// Hello, world!"#
        );
    }

    #[test]
    fn test_render_module() {
        assert_eq!(
            RSDoc::new("Hello, world!", true)
                .render(&rs_indent(), &Default::default())
                .unwrap(),
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
            .render(&rs_indent(), &Default::default())
            .unwrap(),
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
            .render(&rs_indent().increment(), &Default::default())
            .unwrap(),
            r#"    /// Hello,
    /// cruel
    /// world!"#
        );
    }
}
