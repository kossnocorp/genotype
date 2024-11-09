use crate::*;

use super::RSTuple;

impl RSContextResolve for RSTuple {
    fn resolve<Context>(self, _context: &mut Context) -> Self
    where
        Context: RSContext,
    {
        self
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use mock::RSContextMock;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_resolve() {
        let mut context = RSContextMock::default();
        let tuple = RSTuple {
            descriptors: vec![RSPrimitive::String.into()],
        };
        tuple.resolve(&mut context);
        assert_eq!(context.as_imports(), vec![]);
    }
}
