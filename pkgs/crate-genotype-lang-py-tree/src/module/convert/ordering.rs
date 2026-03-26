use crate::prelude::internal::*;
use petgraph::algo::tarjan_scc;
use petgraph::graph::{Graph, NodeIndex};
use std::collections::{HashMap, HashSet};

impl PyConvertModule {
    pub fn sort_definitions(unordered_definitions: Vec<PyDefinition>) -> Vec<PyDefinition> {
        let mut unordered_definitions = unordered_definitions;

        let definition_identifiers = unordered_definitions
            .iter()
            .map(|definition| definition.name().clone())
            .collect::<HashSet<PyIdentifier>>();

        let mut available = HashSet::new();

        let mut definitions = vec![];

        loop {
            if unordered_definitions.is_empty() {
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

            // Nothing found, the remaining definitions are in circular dependency groups.
            break;
        }

        definitions.extend(Self::sort_cyclic_definitions(unordered_definitions));

        definitions
    }

    fn sort_cyclic_definitions(unordered_definitions: Vec<PyDefinition>) -> Vec<PyDefinition> {
        if unordered_definitions.is_empty() {
            return vec![];
        }

        let mut graph: Graph<usize, (), petgraph::Directed> = Graph::new();
        let mut graph_nodes: Vec<NodeIndex> = Vec::with_capacity(unordered_definitions.len());

        for index in 0..unordered_definitions.len() {
            graph_nodes.push(graph.add_node(index));
        }

        let definition_identifiers = unordered_definitions
            .iter()
            .enumerate()
            .map(|(index, definition)| (definition.name().clone(), index))
            .collect::<HashMap<PyIdentifier, usize>>();

        for (definition_index, definition) in unordered_definitions.iter().enumerate() {
            for reference in definition.references() {
                if let Some(reference_index) = definition_identifiers.get(reference) {
                    graph.add_edge(
                        graph_nodes[*reference_index],
                        graph_nodes[definition_index],
                        (),
                    );
                }
            }
        }

        let mut components = tarjan_scc(&graph);
        components.reverse();

        let mut ordered_indices = vec![];

        for component in components {
            if component.len() == 1 {
                ordered_indices.push(graph[component[0]]);
                continue;
            }

            let component_set = component.iter().copied().collect::<HashSet<_>>();
            let mut component_indices = component
                .iter()
                .map(|node| graph[*node])
                .collect::<Vec<usize>>();

            component_indices.sort_by(|left, right| {
                let left_dependencies = unordered_definitions[*left]
                    .references()
                    .iter()
                    .filter_map(|reference| definition_identifiers.get(*reference))
                    .filter(|index| component_set.contains(&graph_nodes[**index]))
                    .count();

                let right_dependencies = unordered_definitions[*right]
                    .references()
                    .iter()
                    .filter_map(|reference| definition_identifiers.get(*reference))
                    .filter(|index| component_set.contains(&graph_nodes[**index]))
                    .count();

                left_dependencies
                    .cmp(&right_dependencies)
                    .then_with(|| right.cmp(left))
            });

            ordered_indices.extend(component_indices);
        }

        let mut indexed_definitions = unordered_definitions
            .into_iter()
            .enumerate()
            .collect::<HashMap<usize, PyDefinition>>();

        ordered_indices
            .into_iter()
            .filter_map(|index| indexed_definitions.remove(&index))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    fn definition_names(definitions: &[PyDefinition]) -> Vec<PyIdentifier> {
        definitions
            .iter()
            .map(|definition| definition.name().clone())
            .collect()
    }

    fn class(name: &str, references: Vec<&str>) -> PyDefinition {
        PyDefinition::Class(PyClass {
            doc: None,
            name: name.into(),
            extensions: vec![],
            properties: vec![],
            references: references.into_iter().map(Into::into).collect(),
        })
    }

    #[test]
    fn test_sort_definitions_acyclic() {
        let definitions = vec![
            class("Order", vec!["Book"]),
            class("Book", vec!["Author"]),
            class("Author", vec![]),
        ];

        let sorted = PyConvertModule::sort_definitions(definitions);

        assert_eq!(
            definition_names(&sorted),
            vec!["Author".into(), "Book".into(), "Order".into()],
        );
    }

    #[test]
    fn test_sort_definitions_cycle_keeps_list_item_before_container() {
        let definitions = vec![
            class("JsonAny", vec!["JsonArray", "JsonObject", "JsonProperty"]),
            class("JsonArray", vec!["JsonAny"]),
            class("JsonObject", vec!["JsonProperty"]),
            class("JsonProperty", vec!["JsonAny"]),
        ];

        let sorted = PyConvertModule::sort_definitions(definitions);
        let sorted_names = definition_names(&sorted);

        let object_index = sorted_names
            .iter()
            .position(|name| *name == "JsonObject".into())
            .unwrap();
        let property_index = sorted_names
            .iter()
            .position(|name| *name == "JsonProperty".into())
            .unwrap();

        assert!(property_index < object_index);
    }
}
