use crate::{GTNode, GTNodeParseResult, GTParseError, GTSpan};

use super::{GTIdentifier, GTKey, GTObjectName, GTObjectNameParent, GTResolve};

#[derive(Debug, PartialEq, Clone)]
pub struct GTContext {
    pub resolve: GTResolve,
    pub parents: Vec<GTContextParent>,
}

/// The parent context enum that defines the kind of a parent an object has.
/// It allows building object names from the parents.
#[derive(Debug, PartialEq, Clone)]
pub enum GTContextParent {
    /// An explicitely named alias parent.
    Alias(GTIdentifier),
    /// An anonymous parent, i.e. an union or a nested object.
    Anonymous,
    /// A property parent.
    Property(GTKey),
}

impl GTContext {
    pub fn new() -> Self {
        GTContext {
            resolve: GTResolve::new(),
            parents: vec![],
        }
    }

    pub fn pop_parent(&mut self, span: GTSpan, node: GTNode) -> GTNodeParseResult<()> {
        self.parents
            .pop()
            .ok_or_else(|| GTParseError::Internal(span.clone(), node))?;
        Ok(())
    }

    pub fn resolve_name(&self, span: GTSpan) -> GTNodeParseResult<GTObjectName> {
        let mut keys = vec![];
        let mut anonymous = false;

        // Go through the parents in reverse order to build the object parent name
        for parent in self.parents.iter().rev() {
            match parent {
                // If there's an property parent, we start building the keys path
                GTContextParent::Property(key) => keys.insert(0, key.clone()),

                // If there's an anonymous parent, we mark the object as anonymous
                // which means we'll end up with an anonymous object name.
                GTContextParent::Anonymous => {
                    anonymous = true;
                }

                // If we finally found an alias parent, we can stop building the object name
                // and resolve it.
                GTContextParent::Alias(identifier) => {
                    // There was an anonymous parent of the path, so we need to build the name from
                    // the context.
                    if anonymous {
                        let parent = if keys.len() == 0 {
                            // If there was no keys on the path, then the parent is an alias.
                            GTObjectNameParent::Alias(identifier.clone())
                        } else {
                            // Otherwise the parent is a property.
                            GTObjectNameParent::Property(identifier.clone(), keys)
                        };
                        let identifier = parent.to_identifier(span.clone());
                        return Ok(GTObjectName::Alias(identifier, parent));
                        // return Ok(GTObjectName::Anonymous(span.clone(), parent));
                    } else {
                        // There was no anonymous parent of the path, so it means this name belongs
                        // to the object. We get here, for example, with AliasName = { ... },
                        // where { ... } finds the AliasName parent.
                        return Ok(GTObjectName::Named(identifier.clone()));
                    }
                }
            }
        }

        // Parents are in an invalid state, we can't resolve the object name.
        Err(GTParseError::Internal(span.clone(), GTNode::ObjectName))
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_object_name_named() {
        let context = GTContext {
            resolve: GTResolve::new(),
            parents: vec![
                GTContextParent::Alias(GTIdentifier::new((0, 5).into(), "Hi".into())),
                GTContextParent::Alias(GTIdentifier::new((5, 10).into(), "Hello".into())),
            ],
        };
        assert_eq!(
            context.resolve_name((50, 55).into()).unwrap(),
            GTObjectName::Named(GTIdentifier::new((5, 10).into(), "Hello".into()))
        );
    }

    #[test]
    fn test_object_name_alias() {
        let context = GTContext {
            resolve: GTResolve::new(),
            parents: vec![
                GTContextParent::Alias(GTIdentifier::new((0, 5).into(), "Hi".into())),
                GTContextParent::Alias(GTIdentifier::new((5, 10).into(), "Hello".into())),
                GTContextParent::Anonymous,
            ],
        };
        assert_eq!(
            context.resolve_name((50, 55).into()).unwrap(),
            GTObjectName::Alias(
                GTIdentifier::new((50, 55).into(), "HelloObj".into()),
                GTObjectNameParent::Alias(GTIdentifier::new((5, 10).into(), "Hello".into()))
            )
        );
    }

    #[test]
    fn test_object_name_property() {
        let context = GTContext {
            resolve: GTResolve::new(),
            parents: vec![
                GTContextParent::Alias(GTIdentifier::new((0, 5).into(), "Hi".into())),
                GTContextParent::Alias(GTIdentifier::new((5, 10).into(), "Hello".into())),
                GTContextParent::Anonymous,
                GTContextParent::Property(GTKey::new((10, 15).into(), "cruel".into())),
                GTContextParent::Property(GTKey::new((15, 20).into(), "world".into())),
            ],
        };
        assert_eq!(
            context.resolve_name((50, 55).into()).unwrap(),
            GTObjectName::Alias(
                GTIdentifier::new((50, 55).into(), "HelloCruelWorld".into()),
                GTObjectNameParent::Property(
                    GTIdentifier::new((5, 10).into(), "Hello".into()),
                    vec![
                        GTKey::new((10, 15).into(), "cruel".into()),
                        GTKey::new((15, 20).into(), "world".into())
                    ]
                )
            )
        );
    }
}
