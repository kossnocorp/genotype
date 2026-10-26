use crate::prelude::internal::*;
use petgraph::{algo::tarjan_scc, graph::DiGraph};

impl TsModule {
    pub fn sort_definitions(definitions: Vec<TsDefinition>) -> Vec<TsDefinition> {
        Self::resolve_definition_order(definitions).0
    }

    pub(crate) fn resolve_definition_order(
        definitions: Vec<TsDefinition>,
    ) -> (Vec<TsDefinition>, IndexSet<TsIdentifier>) {
        let def_names: IndexSet<_> = definitions.iter().map(|def| def.name()).collect();
        let defs_order = definitions
            .iter()
            .enumerate()
            .map(|(index, def)| (def.name(), index))
            .collect::<IndexMap<_, _>>();
        let defs_by_name = definitions
            .iter()
            .map(|def| (def.name(), def.clone()))
            .collect::<IndexMap<_, _>>();

        let mut def_deps_by_name = IndexMap::new();
        for def in &definitions {
            let name = def.name();
            let deps_scan = def.scan_dependencies();
            let mut local_deps = deps_scan.dependencies;
            local_deps.retain(|dependency| {
                def_names.contains(dependency)
                    && !def
                        .generics()
                        .is_some_and(|generics| generics.contains(dependency))
            });
            def_deps_by_name.insert(name, local_deps);
        }

        let comps = Self::strongly_connected_components(&def_deps_by_name);
        let recursive = comps
            .iter()
            .filter(|component| {
                component.len() > 1 || def_deps_by_name[&component[0]].contains(&component[0])
            })
            .flatten()
            .cloned()
            .collect();

        let mut comp_idx = IndexMap::new();
        for (index, comp) in comps.iter().enumerate() {
            for ident in comp {
                comp_idx.insert(ident.clone(), index);
            }
        }

        let mut incoming_count = vec![0usize; comps.len()];
        let mut outgoing = vec![IndexSet::<usize>::new(); comps.len()];

        for (def_name, deps) in &def_deps_by_name {
            let &name_component = comp_idx.get(def_name).unwrap();

            for dep in deps {
                let &dependency_component = comp_idx.get(dep).unwrap();
                if dependency_component == name_component {
                    continue;
                }

                if outgoing[dependency_component].insert(name_component) {
                    incoming_count[name_component] += 1;
                }
            }
        }

        let mut ready_comps = (0..comps.len())
            .filter(|index| incoming_count[*index] == 0)
            .collect::<Vec<_>>();
        ready_comps.sort_by_key(|index| {
            comps[*index]
                .iter()
                .map(|identifier| *defs_order.get(identifier).unwrap())
                .min()
                .unwrap_or(usize::MAX)
        });

        let mut sorted_defs = vec![];
        let mut queued_comp_idxs = IndexSet::new();

        while let Some(comp_idx) = ready_comps.first().cloned() {
            ready_comps.remove(0);
            queued_comp_idxs.insert(comp_idx);

            let component_definitions = Self::sort_component_definitions(
                &comps[comp_idx],
                &def_deps_by_name,
                &defs_by_name,
                &defs_order,
            );
            sorted_defs.extend(component_definitions);

            for dependent in outgoing[comp_idx].clone() {
                incoming_count[dependent] -= 1;
                if incoming_count[dependent] == 0 {
                    ready_comps.push(dependent);
                }
            }

            ready_comps.sort_by_key(|index| {
                comps[*index]
                    .iter()
                    .map(|identifier| *defs_order.get(identifier).unwrap())
                    .min()
                    .unwrap_or(usize::MAX)
            });
        }

        if sorted_defs.len() < definitions.len() {
            for (comp_idx, comp) in comps.iter().enumerate() {
                if queued_comp_idxs.contains(&comp_idx) {
                    continue;
                }

                let comp_defs = Self::sort_component_definitions(
                    comp,
                    &def_deps_by_name,
                    &defs_by_name,
                    &defs_order,
                );
                sorted_defs.extend(comp_defs);
            }
        }

        (sorted_defs, recursive)
    }

    fn sort_component_definitions(
        component: &[TsIdentifier],
        dependencies_by_name: &IndexMap<TsIdentifier, IndexSet<TsIdentifier>>,
        defs_by_name: &IndexMap<TsIdentifier, TsDefinition>,
        order: &IndexMap<TsIdentifier, usize>,
    ) -> Vec<TsDefinition> {
        if component.len() <= 1 {
            return component
                .iter()
                .map(|identifier| defs_by_name.get(identifier).unwrap().clone())
                .collect();
        }

        let comp_set = component.iter().cloned().collect::<IndexSet<_>>();
        let mut sorted_idents = component.to_vec();

        sorted_idents.sort_by(|left, right| {
            let left_deps = dependencies_by_name
                .get(left)
                .into_iter()
                .flatten()
                .filter(|identifier| comp_set.contains(*identifier))
                .count();
            let right_deps = dependencies_by_name
                .get(right)
                .into_iter()
                .flatten()
                .filter(|identifier| comp_set.contains(*identifier))
                .count();

            left_deps.cmp(&right_deps).then_with(|| {
                order
                    .get(right)
                    .unwrap_or(&usize::MAX)
                    .cmp(order.get(left).unwrap_or(&usize::MAX))
            })
        });

        // Properties can be suspended, but spreading an extension's fields is
        // eager. Respect those edges even inside a recursive component.
        let mut sorted = Vec::new();
        while !sorted_idents.is_empty() {
            let next = sorted_idents.iter().position(|name| {
                let TsDefinition::Interface(interface) = &defs_by_name[name] else {
                    return true;
                };
                interface
                    .extensions
                    .iter()
                    .all(|extension| !sorted_idents.contains(&extension.reference.identifier))
            });
            // A cycle consisting entirely of extensions has no valid ordering.
            // Preserve the existing deterministic order for that case.
            let name = sorted_idents.remove(next.unwrap_or(0));
            sorted.push(defs_by_name[&name].clone());
        }
        sorted
    }

    fn strongly_connected_components(
        dependencies: &IndexMap<TsIdentifier, IndexSet<TsIdentifier>>,
    ) -> Vec<Vec<TsIdentifier>> {
        let mut graph = DiGraph::<TsIdentifier, ()>::new();
        let nodes = dependencies
            .keys()
            .map(|name| (name.clone(), graph.add_node(name.clone())))
            .collect::<IndexMap<_, _>>();
        for (name, references) in dependencies {
            for reference in references {
                graph.add_edge(nodes[name], nodes[reference], ());
            }
        }
        tarjan_scc(&graph)
            .into_iter()
            .map(|component| {
                component
                    .into_iter()
                    .map(|node| graph[node].clone())
                    .collect()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_order_and_cycle_membership() {
        let (definitions, recursive) = TsModule::resolve_definition_order(vec_into![
            Tst::alias("Dependent", Tst::reference("A")),
            TsInterface {
                extensions: vec![Tst::extension("B")],
                ..Tst::interface("A", vec![])
            },
            Tst::interface("B", vec![Tst::property("a", Tst::reference("A"))]),
            Tst::alias("Self", Tst::array(Tst::reference("Self"))),
            Tst::alias("Independent", Tst::primitive_string()),
        ]);
        assert_eq!(
            recursive,
            IndexSet::from([
                Tst::identifier("A"),
                Tst::identifier("B"),
                Tst::identifier("Self")
            ])
        );
        assert_eq!(
            definitions
                .iter()
                .map(|definition| definition.name())
                .collect::<Vec<_>>(),
            vec![
                Tst::identifier("B"),
                Tst::identifier("A"),
                Tst::identifier("Dependent"),
                Tst::identifier("Self"),
                Tst::identifier("Independent")
            ]
        );
    }

    #[test]
    fn test_resolve_order_ignores_imports_and_shadowed_generics() {
        let (definitions, recursive) = TsModule::resolve_definition_order(vec_into![
            Tst::alias_with_generics("Generic", vec!["Generic"], Tst::reference("Generic")),
            Tst::alias("Imported", Tst::reference("External")),
            Tst::alias("User", Tst::reference("Base")),
            Tst::alias("Base", Tst::primitive_string()),
        ]);
        assert!(recursive.is_empty());
        assert_eq!(
            definitions
                .iter()
                .map(|definition| definition.name())
                .collect::<Vec<_>>(),
            vec![
                Tst::identifier("Generic"),
                Tst::identifier("Imported"),
                Tst::identifier("Base"),
                Tst::identifier("User")
            ]
        );
    }

    #[test]
    fn test_resolve_empty_module() {
        let (definitions, recursive) = TsModule::resolve_definition_order(vec![]);
        assert!(definitions.is_empty());
        assert!(recursive.is_empty());
    }
}
