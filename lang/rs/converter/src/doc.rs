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
    use pretty_assertions::assert_eq;

    #[test]
    fn test_convert() {
        assert_eq!(
            RSDoc::new("Hello, world!", false),
            GTDoc((0, 0).into(), "Hello, world!".into())
                .convert(&mut RSConvertContext::empty("module".into()))
                .unwrap(),
        );
    }
}
