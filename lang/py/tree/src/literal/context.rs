use crate::{PYContext, PYContextResolve, PYOptions};

use super::PYLiteral;

impl PYContextResolve for PYLiteral {
    fn resolve(self, context: &mut PYContext, _options: &PYOptions) -> Self {
        context.imports.insert(("typing".into(), "Literal".into()));
        self
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_resolve() {
        let mut context = PYContext::new();
        let literal = PYLiteral::Boolean(true);
        literal.resolve(&mut context, &PYOptions::default());
        assert_eq!(
            context,
            PYContext {
                imports: HashSet::from_iter(vec![("typing".into(), "Literal".into())]),
            }
        );
    }
}
