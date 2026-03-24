use crate::prelude::internal::*;

impl RSConvert<RSDoc> for GTDoc {
    fn convert(&self, _context: &mut RSConvertContext) -> Result<RSDoc> {
        Ok(RSDoc(
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
            GTDoc((0, 0).into(), "Hello, world!".into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
            @r#"RSDoc("Hello, world!", false)"#
        );
    }
}
