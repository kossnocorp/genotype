use crate::prelude::internal::*;

impl PYConvert<PYDoc> for GTDoc {
    fn convert(&self, _context: &mut PYConvertContext) -> PYDoc {
        PYDoc(self.1.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        assert_eq!(
            PYDoc("Hello, world!".into()),
            GTDoc((0, 0).into(), "Hello, world!".into()).convert(&mut PYConvertContext::default()),
        );
    }
}
