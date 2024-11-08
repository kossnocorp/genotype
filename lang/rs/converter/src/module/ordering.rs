use genotype_lang_rs_tree::{RSDefinition, RSIdentifier};
use std::collections::HashSet;

use super::RSConvertModule;

impl RSConvertModule {
    pub fn sort_definitions(unordered_definitions: Vec<RSDefinition>) -> Vec<RSDefinition> {
        let mut unordered_definitions = unordered_definitions;

        let definition_identifiers = unordered_definitions
            .iter()
            .map(|definition| definition.name().clone())
            .collect::<HashSet<RSIdentifier>>();

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
