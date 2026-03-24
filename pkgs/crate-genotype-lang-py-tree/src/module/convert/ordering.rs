use crate::prelude::internal::*;
use std::collections::HashSet;

impl PYConvertModule {
    pub fn sort_definitions(unordered_definitions: Vec<PYDefinition>) -> Vec<PYDefinition> {
        let mut unordered_definitions = unordered_definitions;

        let definition_identifiers = unordered_definitions
            .iter()
            .map(|definition| definition.name().clone())
            .collect::<HashSet<PYIdentifier>>();

        let mut available = HashSet::new();

        let mut definitions = vec![];

        loop {
            if unordered_definitions.len() == 0 {
                break;
            }

            let next_index = unordered_definitions.iter().position(|definition| {
                definition.references().iter().all(|reference| {
                    !definition_identifiers.contains(reference) || available.contains(*reference)
                })
            });

            if let Some(index) = next_index {
                let definition = unordered_definitions.remove(index);
                available.insert(definition.name().clone());
                definitions.push(definition);
                continue;
            }

            // Nothing found, the remaining definitions are in a circular dependency
            break;
        }

        // [TODO] Do something with the circular dependencies
        definitions.extend(unordered_definitions);

        definitions
    }
}
