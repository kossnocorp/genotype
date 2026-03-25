use crate::prelude::internal::*;

impl RsConvert<RsDoc> for GtDoc {
    fn convert(&self, _context: &mut RsConvertContext) -> Result<RsDoc> {
        Ok(RsDoc(
            self.1.clone(),
            false, // It is assigned by module converter
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use insta::assert_ron_snapshot;

    #[test]
    fn test_convert() {
        assert_ron_snapshot!(
            GtDoc((0, 0).into(), "Hello, world!".into())
                .convert(&mut RsConvertContext::empty("module".into()))
                .unwrap(),
            @r#"RsDoc("Hello, world!", false)"#
        );
    }
}
