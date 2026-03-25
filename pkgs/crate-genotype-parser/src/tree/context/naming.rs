use crate::prelude::internal::*;

impl GtContext {
    pub fn enter_parent(&mut self, parent: GtContextParent) {
        self.parents.push(parent)
    }

    pub fn exit_parent(&mut self, span: GtSpan, node: GtNode) -> GtNodeParseResult<()> {
        self.parents
            .pop()
            .ok_or_else(|| GtParseError::Internal(span.clone(), node))?;
        Ok(())
    }

    /// Builds the object name from the parents.
    pub fn name_object(&mut self, span: GtSpan) -> GtNodeParseResult<GtObjectName> {
        // The alias is the immediate parent, so we can return it.
        if let Some(name) = self.claim_alias() {
            self.claim_name(name.1.as_ref());
            return Ok(GtObjectName::Named(name));
        }

        let mut keys = vec![];

        // Go through the parents in reverse order to build the name
        for parent in self.parents.iter().rev() {
            match parent {
                // If there's an property parent, we start building the keys path
                GtContextParent::Property(key) => keys.insert(0, key.clone()),

                // If we finally found an alias parent, we can stop building the name and retuen it.
                GtContextParent::Alias(identifier) => {
                    let parent = if keys.len() == 0 {
                        // If there was no keys on the path, then the parent is an alias.
                        GtObjectNameParent::Alias(identifier.clone())
                    } else {
                        // Otherwise the parent is a property.
                        GtObjectNameParent::Property(identifier.clone(), keys)
                    };

                    let identifier = parent.to_identifier(span.clone());
                    // [TODO] Ensure unique name
                    self.claim_name(identifier.1.as_ref());

                    return Ok(GtObjectName::Alias(identifier, parent));
                }

                // Nothing to do
                GtContextParent::Anonymous => {}
            }
        }

        // Parents are in an invalid state, we can't resolve the object name.
        Err(GtParseError::Internal(span.clone(), GtNode::ObjectName))
    }

    /// Tries claiming the alias from the parent.
    pub fn claim_alias(&self) -> Option<GtIdentifier> {
        if let Some(GtContextParent::Alias(identifier)) = self.parents.last() {
            return Some(identifier.clone());
        }
        None
    }

    /// It generates a definition id and name from the parents.
    pub fn get_name(&mut self, span: &GtSpan, base_name: &str) -> GtIdentifier {
        let name = if let Some(name) = self.claim_alias() {
            // The alias is the immediate parent, so we can return it.
            name
        } else {
            // If the immediate parent is an anonymous parent, we'll have to use the base name.
            let anonymous = if let Some(GtContextParent::Anonymous) = self.parents.last() {
                true
            } else {
                false
            };

            let mut segments: Vec<String> = vec![];
            for parent in self.parents.iter().rev() {
                match parent {
                    // Add any keys on the path to the name segments.
                    GtContextParent::Property(key) => segments.push(key.1.to_string()),

                    // If we finally found an alias parent, we can stop building the name.
                    GtContextParent::Alias(identifier) => {
                        segments.push(identifier.1.to_string());
                        break;
                    }

                    // Ignore anonymous parents.
                    GtContextParent::Anonymous => {}
                }
            }

            segments.reverse();

            if segments.len() == 0 || anonymous {
                segments.push(base_name.to_string().to_pascal_case());
            }

            let name = segments.join("").into();
            self.ensure_unique_name(span, name)
        };

        self.claim_name(name.1.as_ref());

        name
    }

    /// Enumerates the name if it's already claimed.
    fn ensure_unique_name(&self, span: &GtSpan, name: String) -> GtIdentifier {
        let name = if self.is_name_claimed(&name) {
            self.enumerate_name(&name)
        } else {
            name
        };

        GtIdentifier::new(span.clone(), name.into())
    }

    /// Enumerates the name if it's already claimed.
    fn enumerate_name(&self, name: &str) -> String {
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
    fn claim_name(&mut self, name: &str) {
        self.claimed_names.insert(name.to_string());
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_object_name_named() {
        let mut context = GtContext {
            module_id: "module".into(),
            resolve: GtModuleResolve::new(),
            parents: vec![
                GtContextParent::Alias(GtIdentifier::new((0, 5).into(), "Hi".into())),
                GtContextParent::Alias(GtIdentifier::new((5, 10).into(), "Hello".into())),
            ],
            claimed_names: Default::default(),
            annotation: None,
        };
        assert_eq!(
            context.name_object((50, 55).into()).unwrap(),
            GtObjectName::Named(GtIdentifier::new((5, 10).into(), "Hello".into()))
        );
    }

    #[test]
    fn test_object_name_alias() {
        let mut context = GtContext {
            module_id: "module".into(),
            resolve: GtModuleResolve::new(),
            parents: vec![
                GtContextParent::Alias(GtIdentifier::new((0, 5).into(), "Hi".into())),
                GtContextParent::Alias(GtIdentifier::new((5, 10).into(), "Hello".into())),
                GtContextParent::Anonymous,
            ],
            claimed_names: Default::default(),
            annotation: None,
        };
        assert_eq!(
            context.name_object((50, 55).into()).unwrap(),
            GtObjectName::Alias(
                GtIdentifier::new((50, 55).into(), "HelloObj".into()),
                GtObjectNameParent::Alias(GtIdentifier::new((5, 10).into(), "Hello".into()))
            )
        );
    }

    #[test]
    fn test_object_name_property() {
        let mut context = GtContext {
            module_id: "module".into(),
            resolve: GtModuleResolve::new(),
            parents: vec![
                GtContextParent::Alias(GtIdentifier::new((0, 5).into(), "Hi".into())),
                GtContextParent::Alias(GtIdentifier::new((5, 10).into(), "Hello".into())),
                GtContextParent::Anonymous,
                GtContextParent::Property(GtKey::new((10, 15).into(), "cruel".into())),
                GtContextParent::Property(GtKey::new((15, 20).into(), "world".into())),
            ],
            claimed_names: Default::default(),
            annotation: None,
        };
        assert_eq!(
            context.name_object((50, 55).into()).unwrap(),
            GtObjectName::Alias(
                GtIdentifier::new((50, 55).into(), "HelloCruelWorld".into()),
                GtObjectNameParent::Property(
                    GtIdentifier::new((5, 10).into(), "Hello".into()),
                    vec![
                        GtKey::new((10, 15).into(), "cruel".into()),
                        GtKey::new((15, 20).into(), "world".into())
                    ]
                )
            )
        );
    }
}
