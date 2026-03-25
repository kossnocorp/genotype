use crate::prelude::internal::*;

impl PyConvert<PyDoc> for GtDoc {
    fn convert(&self, _context: &mut PyConvertContext) -> PyDoc {
        PyDoc(self.1.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            GtDoc((0, 0).into(), "Hello, world!".into()).convert(&mut PyConvertContext::default()),
            @r#"PyDoc("Hello, world!")"#
        );
    }
}
