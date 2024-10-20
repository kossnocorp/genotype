use genotype_config::GTConfig;
use genotype_lang_core_tree::{indent::GTIndent, render::GTRender};
use genotype_lang_py_config::PYVersion;

use crate::PYRender;

use super::PYReference;

impl PYRender for PYReference {
    fn render(&self, indent: &GTIndent, config: &GTConfig) -> String {
        let str = self.identifier.render(indent);
        if let PYVersion::Legacy = config.python_version() {
            if self.forward {
                return format!("\"{}\"", str);
            }
        }
        str
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_py_config::PYConfig;

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
            PYReference::new("Foo".into(), true).render(
                &py_indent(),
                &GTConfig::default().with_python(PYConfig::new(PYVersion::Legacy))
            ),
            "\"Foo\""
        );
    }
}
