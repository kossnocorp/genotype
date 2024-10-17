use crate::*;

use super::PYProperty;

impl PYContextResolve for PYProperty {
    fn resolve(self, context: &mut PYContext, _options: &PYOptions) -> Self {
        if !self.required {
            context.imports.insert(("typing".into(), "Optional".into()));
        }
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
        let alias = PYProperty {
            name: "foo".into(),
            descriptor: PYPrimitive::String.into(),
            required: true,
        };
        alias.resolve(&mut context, &PYOptions::default());
        assert_eq!(context, PYContext::new());
    }

    #[test]
    fn test_resolve_optional() {
        let mut context = PYContext::new();
        let alias = PYProperty {
            name: "foo".into(),
            descriptor: PYPrimitive::String.into(),
            required: false,
        };
        alias.resolve(&mut context, &PYOptions::default());
        assert_eq!(
            context,
            PYContext {
                imports: HashSet::from_iter(vec![("typing".into(), "Optional".into())]),
            }
        );
    }
}
