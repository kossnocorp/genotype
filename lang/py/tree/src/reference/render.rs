use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};

use crate::{PYOptions, PYRender, PYVersion};

use super::PYReference;

impl PYRender for PYReference {
    fn render(&self, indent: &GTIndent, options: &PYOptions) -> String {
        let str = self.identifier.render(indent);
        if let PYVersion::Legacy = options.version {
            if self.forward {
                return format!("\"{}\"", str);
            }
        }
        str
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{indent::py_indent, PYVersion};

    #[test]
    fn test_render() {
        assert_eq!(
            PYReference::new("Foo".into(), false).render(&py_indent(), &PYOptions::default()),
            "Foo"
        );
    }

    #[test]
    fn test_render_forward() {
        assert_eq!(
            PYReference::new("Foo".into(), true).render(&py_indent(), &PYOptions::default()),
            "Foo"
        );
        assert_eq!(
            PYReference::new("Foo".into(), true)
                .render(&py_indent(), &PYOptions::new(PYVersion::Legacy)),
            "\"Foo\""
        );
    }
}
