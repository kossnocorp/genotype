use crate::prelude::internal::*;

impl TsConvertModule {
    pub fn sort_definitions(definitions: Vec<TsDefinition>) -> Vec<TsDefinition> {
        let def_names: HashSet<_> = definitions.iter().map(|def| def.name()).collect();
        let defs_order = definitions
            .iter()
            .enumerate()
            .map(|(index, def)| (def.name(), index))
            .collect::<HashMap<_, _>>();
        let defs_by_name = definitions
            .iter()
            .map(|def| (def.name(), def.clone()))
            .collect::<HashMap<_, _>>();

        let mut def_deps_by_name = HashMap::new();
        for def in &definitions {
            let name = def.name();
            let deps_scan = def.scan_dependencies();
            let mut local_deps = deps_scan.dependencies;
            local_deps.retain(|dependency| def_names.contains(dependency));
            def_deps_by_name.insert(name, local_deps);
        }

        let comps = Self::strongly_connected_components(&def_deps_by_name, &defs_order);

        let mut comp_idx = HashMap::new();
        for (index, comp) in comps.iter().enumerate() {
            for ident in comp {
                comp_idx.insert(ident.clone(), index);
            }
        }

        let mut incoming_count = vec![0usize; comps.len()];
        let mut outgoing = vec![HashSet::<usize>::new(); comps.len()];

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
        let mut queued_comp_idxs = HashSet::new();

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

        sorted_defs
    }

    fn sort_component_definitions(
        component: &[TsIdentifier],
        dependencies_by_name: &HashMap<TsIdentifier, HashSet<TsIdentifier>>,
        defs_by_name: &HashMap<TsIdentifier, TsDefinition>,
        order: &HashMap<TsIdentifier, usize>,
    ) -> Vec<TsDefinition> {
        if component.len() <= 1 {
            return component
                .iter()
                .map(|identifier| defs_by_name.get(identifier).unwrap().clone())
                .collect();
        }

        let comp_set = component.iter().cloned().collect::<HashSet<_>>();
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

        sorted_idents
            .into_iter()
            .map(|ident| defs_by_name.get(&ident).unwrap().clone())
            .collect()
    }

    fn strongly_connected_components(
        deps_by_name: &HashMap<TsIdentifier, HashSet<TsIdentifier>>,
        order: &HashMap<TsIdentifier, usize>,
    ) -> Vec<Vec<TsIdentifier>> {
        let mut idx = 0usize;
        let mut stack: Vec<TsIdentifier> = vec![];
        let mut on_stack: HashSet<TsIdentifier> = HashSet::new();
        let mut idxs: HashMap<TsIdentifier, usize> = HashMap::new();
        let mut low_links: HashMap<TsIdentifier, usize> = HashMap::new();
        let mut comps: Vec<Vec<TsIdentifier>> = vec![];

        // TODO: Get rid of this mess here!
        #[allow(clippy::too_many_arguments)]
        fn visit(
            node: TsIdentifier,
            idx: &mut usize,
            stack: &mut Vec<TsIdentifier>,
            on_stack: &mut HashSet<TsIdentifier>,
            idxs: &mut HashMap<TsIdentifier, usize>,
            low_links: &mut HashMap<TsIdentifier, usize>,
            deps_by_name: &HashMap<TsIdentifier, HashSet<TsIdentifier>>,
            comps: &mut Vec<Vec<TsIdentifier>>,
        ) {
            idxs.insert(node.clone(), *idx);
            low_links.insert(node.clone(), *idx);
            *idx += 1;

            stack.push(node.clone());
            on_stack.insert(node.clone());

            for dep in deps_by_name.get(&node).into_iter().flatten() {
                if !idxs.contains_key(dep) {
                    visit(
                        dep.clone(),
                        idx,
                        stack,
                        on_stack,
                        idxs,
                        low_links,
                        deps_by_name,
                        comps,
                    );

                    let node_low = *low_links.get(&node).unwrap();
                    let dep_low = *low_links.get(dep).unwrap();
                    low_links.insert(node.clone(), node_low.min(dep_low));
                } else if on_stack.contains(dep) {
                    let node_low = *low_links.get(&node).unwrap();
                    let dep_idx = *idxs.get(dep).unwrap();
                    low_links.insert(node.clone(), node_low.min(dep_idx));
                }
            }

            if idxs.get(&node) == low_links.get(&node) {
                let mut comp = vec![];

                loop {
                    let Some(component_node) = stack.pop() else {
                        break;
                    };

                    on_stack.remove(&component_node);
                    comp.push(component_node.clone());

                    if component_node == node {
                        break;
                    }
                }

                comps.push(comp);
            }
        }

        let mut names = deps_by_name.keys().cloned().collect::<Vec<_>>();
        names.sort_by_key(|name| *order.get(name).unwrap_or(&usize::MAX));

        for name in names {
            if idxs.contains_key(&name) {
                continue;
            }

            visit(
                name,
                &mut idx,
                &mut stack,
                &mut on_stack,
                &mut idxs,
                &mut low_links,
                deps_by_name,
                &mut comps,
            );
        }

        comps
    }
}
