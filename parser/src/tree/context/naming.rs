use heck::ToPascalCase;

use crate::{
    GTIdentifier, GTNode, GTNodeParseResult, GTObjectName, GTObjectNameParent, GTParseError, GTSpan,
};

use super::{GTContext, GTContextParent};

impl GTContext {
    pub fn enter_parent(&mut self, parent: GTContextParent) {
        self.parents.push(parent)
    }

    pub fn exit_parent(&mut self, span: GTSpan, node: GTNode) -> GTNodeParseResult<()> {
        self.parents
            .pop()
            .ok_or_else(|| GTParseError::Internal(span.clone(), node))?;
        Ok(())
    }

    /// Builds the object name from the parents.
    pub fn name_object(&mut self, span: GTSpan) -> GTNodeParseResult<GTObjectName> {
        // The alias is the immediate parent, so we can return it.
        if let Some(name) = self.claim_alias() {
            self.claim_name(&name.1);
            return Ok(GTObjectName::Named(name));
        }

        let mut keys = vec![];

        // Go through the parents in reverse order to build the name
        for parent in self.parents.iter().rev() {
            match parent {
                // If there's an property parent, we start building the keys path
                GTContextParent::Property(key) => keys.insert(0, key.clone()),

                // If we finally found an alias parent, we can stop building the name and retuen it.
                GTContextParent::Alias(identifier) => {
                    let parent = if keys.len() == 0 {
                        // If there was no keys on the path, then the parent is an alias.
                        GTObjectNameParent::Alias(identifier.clone())
                    } else {
                        // Otherwise the parent is a property.
                        GTObjectNameParent::Property(identifier.clone(), keys)
                    };

                    let identifier = parent.to_identifier(span.clone());
                    // [TODO] Ensure unique name
                    self.claim_name(&identifier.1);

                    return Ok(GTObjectName::Alias(identifier, parent));
                }

                // Nothing to do
                GTContextParent::Anonymous => {}
            }
        }

        // Parents are in an invalid state, we can't resolve the object name.
        Err(GTParseError::Internal(span.clone(), GTNode::ObjectName))
    }

    /// Tries claiming the alias from the parent.
    pub fn claim_alias(&self) -> Option<GTIdentifier> {
        if let Some(GTContextParent::Alias(identifier)) = self.parents.last() {
            return Some(identifier.clone());
        }
        None
    }

    /// It generates a definition id and name from the parents.
    pub fn get_name(&mut self, span: &GTSpan, base_name: &str) -> GTIdentifier {
        let name = if let Some(name) = self.claim_alias() {
            // The alias is the immediate parent, so we can return it.
            name
        } else {
            // If the immediate parent is an anonymous parent, we'll have to use the base name.
            let anonymous = if let Some(GTContextParent::Anonymous) = self.parents.last() {
                true
            } else {
                false
            };

            let mut segments = vec![];
            for parent in self.parents.iter().rev() {
                match parent {
                    // Add any keys on the path to the name segments.
                    GTContextParent::Property(key) => segments.push(key.1.clone()),

                    // If we finally found an alias parent, we can stop building the name.
                    GTContextParent::Alias(identifier) => {
                        segments.push(identifier.1.clone());
                        break;
                    }

                    // Ignore anonymous parents.
                    GTContextParent::Anonymous => {}
                }
            }

            segments.reverse();

            if segments.len() == 0 || anonymous {
                segments.push(base_name.to_string().to_pascal_case());
            }

            let name = segments.join("").into();
            self.ensure_unique_name(span, name)
        };

        self.claim_name(&name.1);

        name
    }

    /// Enumerates the name if it's already claimed.
    fn ensure_unique_name(&self, span: &GTSpan, name: String) -> GTIdentifier {
        let name = if self.is_name_claimed(&name) {
            self.enumerate_name(&name)
        } else {
            name
        };

        GTIdentifier::new(span.clone(), name)
    }

    /// Enumerates the name if it's already claimed.
    fn enumerate_name(&self, name: &String) -> String {
        let mut index = 2;
        loop {
            let enumerated_name = format!("{name}{index}");
            if !self.claimed_names.contains(&enumerated_name) {
                return enumerated_name;
            }
            index += 1;
        }
    }

    /// Checks whether the name is already claimed.
    fn is_name_claimed<Str: AsRef<str>>(&self, name: Str) -> bool {
        self.claimed_names.contains(name.as_ref())
    }

    /// Takes the name so it can't be used again.
    fn claim_name(&mut self, name: &String) {
        self.claimed_names.insert(name.clone());
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_object_name_named() {
        let mut context = GTContext {
            module_id: "module".into(),
            resolve: GTModuleResolve::new(),
            parents: vec![
                GTContextParent::Alias(GTIdentifier::new((0, 5).into(), "Hi".into())),
                GTContextParent::Alias(GTIdentifier::new((5, 10).into(), "Hello".into())),
            ],
            claimed_names: Default::default(),
        };
        assert_eq!(
            context.name_object((50, 55).into()).unwrap(),
            GTObjectName::Named(GTIdentifier::new((5, 10).into(), "Hello".into()))
        );
    }

    #[test]
    fn test_object_name_alias() {
        let mut context = GTContext {
            module_id: "module".into(),
            resolve: GTModuleResolve::new(),
            parents: vec![
                GTContextParent::Alias(GTIdentifier::new((0, 5).into(), "Hi".into())),
                GTContextParent::Alias(GTIdentifier::new((5, 10).into(), "Hello".into())),
                GTContextParent::Anonymous,
            ],
            claimed_names: Default::default(),
        };
        assert_eq!(
            context.name_object((50, 55).into()).unwrap(),
            GTObjectName::Alias(
                GTIdentifier::new((50, 55).into(), "HelloObj".into()),
                GTObjectNameParent::Alias(GTIdentifier::new((5, 10).into(), "Hello".into()))
            )
        );
    }

    #[test]
    fn test_object_name_property() {
        let mut context = GTContext {
            module_id: "module".into(),
            resolve: GTModuleResolve::new(),
            parents: vec![
                GTContextParent::Alias(GTIdentifier::new((0, 5).into(), "Hi".into())),
                GTContextParent::Alias(GTIdentifier::new((5, 10).into(), "Hello".into())),
                GTContextParent::Anonymous,
                GTContextParent::Property(GTKey::new((10, 15).into(), "cruel".into())),
                GTContextParent::Property(GTKey::new((15, 20).into(), "world".into())),
            ],
            claimed_names: Default::default(),
        };
        assert_eq!(
            context.name_object((50, 55).into()).unwrap(),
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
