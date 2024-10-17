use crate::*;

use super::PYUnion;

impl PYContextResolve for PYUnion {
    fn resolve(self, context: &mut PYContext, options: &PYOptions) -> Self {
        if let PYVersion::Legacy = options.version {
            context.imports.insert(("typing".into(), "Union".into()));
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
        let union = PYUnion {
            descriptors: vec![PYPrimitive::String.into()],
        };
        union.resolve(&mut context, &PYOptions::default());
        assert_eq!(context, PYContext::new());
    }

    #[test]
    fn test_resolve_legacy() {
        let mut context = PYContext::new();
        let union = PYUnion {
            descriptors: vec![PYPrimitive::String.into()],
        };
        union.resolve(&mut context, &PYOptions::new(PYVersion::Legacy));
        assert_eq!(
            context,
            PYContext {
                imports: HashSet::from_iter(vec![("typing".into(), "Union".into())]),
            }
        );
    }
}
