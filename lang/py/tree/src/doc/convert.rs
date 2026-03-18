use crate::prelude::internal::*;

impl PYConvert<PYDoc> for GTDoc {
    fn convert(&self, _context: &mut PYConvertContext) -> PYDoc {
        PYDoc(self.1.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            GTDoc((0, 0).into(), "Hello, world!".into()).convert(&mut PYConvertContext::default()),
            @r#"PYDoc("Hello, world!")"#
        );
    }
}
