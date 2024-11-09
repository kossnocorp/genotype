use crate::*;

use super::RSVec;

impl RSContextResolve for RSVec {
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
        let list = RSVec {
            descriptor: RSPrimitive::String.into(),
        };
        list.resolve(&mut context);
        assert_eq!(context.as_imports(), vec![]);
    }
}
