use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};
use genotype_lang_py_config::PYLangConfig;
use genotype_lang_py_config::PYVersion;

use crate::PYRender;

use super::PYReference;

impl PYRender for PYReference {
    fn render(&self, indent: &GTIndent, config: &PYLangConfig) -> String {
        let str = self.identifier.render(indent);
        if let PYVersion::Legacy = config.version {
            if self.forward {
                return format!("\"{}\"", str);
            }
        }
        str
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_py_config::PYLangConfig;

    use super::*;
    use crate::indent::py_indent;

    #[test]
    fn test_render() {
        assert_eq!(
            PYReference::new("Foo".into(), false).render(&py_indent(), &Default::default()),
            "Foo"
        );
    }

    #[test]
    fn test_render_forward() {
        assert_eq!(
            PYReference::new("Foo".into(), true).render(&py_indent(), &Default::default()),
            "Foo"
        );
        assert_eq!(
            PYReference::new("Foo".into(), true)
                .render(&py_indent(), &PYLangConfig::new(PYVersion::Legacy)),
            "\"Foo\""
        );
    }
}
