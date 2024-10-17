use crate::{GTNode, GTNodeParseError, GTNodeParseResult, GTSpan};

use super::{GTIdentifier, GTKey, GTObjectName, GTObjectNameParent, GTResolve};

#[derive(Debug, PartialEq, Clone)]
pub struct GTContext {
    pub resolve: GTResolve,
    pub parents: Vec<GTContextParent>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum GTContextParent {
    Alias(GTIdentifier),
    Anonymous,
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
            .ok_or_else(|| GTNodeParseError::Internal(span.clone(), node))?;
        Ok(())
    }

    pub fn object_parent(&self, span: GTSpan) -> GTNodeParseResult<GTObjectName> {
        let mut keys = vec![];
        let mut anonymous = false;

        for parent in self.parents.iter().rev() {
            match parent {
                GTContextParent::Property(key) => keys.insert(0, key.clone()),

                GTContextParent::Anonymous => {
                    anonymous = true;
                }

                GTContextParent::Alias(identifier) => {
                    if anonymous {
                        return Ok(GTObjectName::Anonymous(
                            span.clone(),
                            if keys.len() == 0 {
                                GTObjectNameParent::Alias(identifier.clone())
                            } else {
                                GTObjectNameParent::Property(identifier.clone(), keys)
                            },
                        ));
                    } else {
                        return Ok(GTObjectName::Named(identifier.clone()));
                    }
                }
            }
        }

        Err(GTNodeParseError::Internal(span.clone(), GTNode::ObjectName))
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
            context.object_parent((50, 55).into()).unwrap(),
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
            context.object_parent((50, 55).into()).unwrap(),
            GTObjectName::Anonymous(
                (50, 55).into(),
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
            context.object_parent((50, 55).into()).unwrap(),
            GTObjectName::Anonymous(
                (50, 55).into(),
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
