use crate::prelude::internal::*;
use petgraph::algo::tarjan_scc;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::visit::EdgeRef;

impl RsProjectModule {
    pub fn box_modules(
        project_modules: &mut IndexMap<GtpModulePath, GtlProjectModuleState<RsProjectModule>>,
    ) {
        let mut graph: Graph<GtDefinitionId, (), petgraph::Directed> = Graph::new();
        let mut graph_nodes: IndexMap<GtDefinitionId, NodeIndex> = Default::default();

        for (_, module_state) in project_modules.iter() {
            if let Some(project_module) = module_state.project_module() {
                for definition in project_module.module.definitions.iter() {
                    let definition_id = definition.id().clone();
                    let node = graph.add_node(definition_id.clone());
                    graph_nodes.insert(definition_id, node);
                }
            }
        }

        for (_, module_state) in project_modules.iter() {
            if let Some(project_module) = module_state.project_module() {
                for definition in project_module.module.definitions.iter() {
                    let definition_id = definition.id();
                    let source_node = match graph_nodes.get(definition_id) {
                        Some(node) => *node,
                        None => continue,
                    };

                    for dependency in Self::definition_direct_dependencies(definition) {
                        if let Some(target_node) = graph_nodes.get(&dependency) {
                            graph.add_edge(source_node, *target_node, ());
                        }
                    }
                }
            }
        }

        let mut recursive_definitions: IndexSet<GtDefinitionId> = Default::default();
        for component in tarjan_scc(&graph) {
            if component.len() > 1 {
                for node in component {
                    recursive_definitions.insert(graph[node].clone());
                }
                continue;
            }

            let Some(node) = component.first() else {
                continue;
            };

            let has_self_loop = graph
                .edges(*node)
                .any(|edge| edge.source() == *node && edge.target() == *node);

            if has_self_loop {
                recursive_definitions.insert(graph[*node].clone());
            }
        }

        if recursive_definitions.is_empty() {
            return;
        }

        for (_, module_state) in project_modules.iter_mut() {
            if let Some(project_module) = module_state.project_module_mut() {
                for definition in project_module.module.definitions.iter_mut() {
                    Self::box_definition_recursive_references(definition, &recursive_definitions);
                }
            }
        }
    }

    fn box_definition_recursive_references(
        definition: &mut RsDefinition,
        recursive_definitions: &IndexSet<GtDefinitionId>,
    ) {
        let definition_id = definition.id().clone();
        let is_recursive_definition = recursive_definitions.contains(&definition_id);
        if !is_recursive_definition {
            return;
        }

        match definition {
            RsDefinition::Alias(alias) => {
                Self::box_descriptor_recursive_references(
                    &mut alias.descriptor,
                    &definition_id,
                    true,
                    recursive_definitions,
                );
            }

            RsDefinition::Struct(r#struct) => match &mut r#struct.fields {
                RsStructFields::Newtype(descriptors) => {
                    for descriptor in descriptors {
                        Self::box_descriptor_recursive_references(
                            descriptor,
                            &definition_id,
                            true,
                            recursive_definitions,
                        );
                    }
                }

                RsStructFields::Resolved(fields) => {
                    for field in fields {
                        Self::box_descriptor_recursive_references(
                            &mut field.descriptor,
                            &definition_id,
                            true,
                            recursive_definitions,
                        );
                    }
                }

                _ => {}
            },

            RsDefinition::Enum(r#enum) => {
                for variant in &mut r#enum.variants {
                    if let Some(RsEnumVariantDescriptor::Descriptor(descriptor)) =
                        &mut variant.descriptor
                    {
                        Self::box_descriptor_recursive_references(
                            descriptor,
                            &definition_id,
                            true,
                            recursive_definitions,
                        );
                    }
                }
            }
        }
    }

    fn box_descriptor_recursive_references(
        descriptor: &mut RsDescriptor,
        current_definition_id: &GtDefinitionId,
        direct: bool,
        recursive_definitions: &IndexSet<GtDefinitionId>,
    ) {
        match descriptor {
            RsDescriptor::Reference(reference)
                if direct
                    && recursive_definitions.contains(&reference.definition_id)
                    && recursive_definitions.contains(current_definition_id) =>
            {
                *descriptor = RsDescriptor::boxed(descriptor.clone());
            }

            RsDescriptor::Option(option) => {
                Self::box_descriptor_recursive_references(
                    &mut option.descriptor,
                    current_definition_id,
                    direct,
                    recursive_definitions,
                );
            }

            RsDescriptor::Tuple(tuple) => {
                for descriptor in &mut tuple.descriptors {
                    Self::box_descriptor_recursive_references(
                        descriptor,
                        current_definition_id,
                        direct,
                        recursive_definitions,
                    );
                }
            }

            RsDescriptor::Enum(r#enum) => {
                for variant in &mut r#enum.variants {
                    if let Some(RsEnumVariantDescriptor::Descriptor(descriptor)) =
                        &mut variant.descriptor
                    {
                        Self::box_descriptor_recursive_references(
                            descriptor,
                            current_definition_id,
                            direct,
                            recursive_definitions,
                        );
                    }
                }
            }

            RsDescriptor::Vec(array) => {
                Self::box_descriptor_recursive_references(
                    &mut array.descriptor,
                    current_definition_id,
                    false,
                    recursive_definitions,
                );
            }

            RsDescriptor::Map(map) => {
                Self::box_descriptor_recursive_references(
                    &mut map.key,
                    current_definition_id,
                    false,
                    recursive_definitions,
                );
                Self::box_descriptor_recursive_references(
                    &mut map.descriptor,
                    current_definition_id,
                    false,
                    recursive_definitions,
                );
            }

            RsDescriptor::Box(inner) => {
                Self::box_descriptor_recursive_references(
                    inner,
                    current_definition_id,
                    false,
                    recursive_definitions,
                );
            }

            _ => {}
        }
    }

    fn definition_direct_dependencies(definition: &RsDefinition) -> IndexSet<GtDefinitionId> {
        let mut dependencies = IndexSet::new();

        match definition {
            RsDefinition::Alias(alias) => {
                Self::collect_descriptor_direct_dependencies(
                    &alias.descriptor,
                    true,
                    &mut dependencies,
                );
            }

            RsDefinition::Struct(r#struct) => match &r#struct.fields {
                RsStructFields::Newtype(descriptors) => {
                    for descriptor in descriptors {
                        Self::collect_descriptor_direct_dependencies(
                            descriptor,
                            true,
                            &mut dependencies,
                        );
                    }
                }
                RsStructFields::Resolved(fields) => {
                    for field in fields {
                        Self::collect_descriptor_direct_dependencies(
                            &field.descriptor,
                            true,
                            &mut dependencies,
                        );
                    }
                }
                _ => {}
            },

            RsDefinition::Enum(r#enum) => {
                for variant in &r#enum.variants {
                    if let Some(RsEnumVariantDescriptor::Descriptor(descriptor)) =
                        &variant.descriptor
                    {
                        Self::collect_descriptor_direct_dependencies(
                            descriptor,
                            true,
                            &mut dependencies,
                        );
                    }
                }
            }
        }

        dependencies
    }

    fn collect_descriptor_direct_dependencies(
        descriptor: &RsDescriptor,
        direct: bool,
        dependencies: &mut IndexSet<GtDefinitionId>,
    ) {
        match descriptor {
            RsDescriptor::Reference(reference) => {
                if direct {
                    dependencies.insert(reference.definition_id.clone());
                }
            }

            RsDescriptor::Option(option) => {
                Self::collect_descriptor_direct_dependencies(
                    &option.descriptor,
                    direct,
                    dependencies,
                );
            }

            RsDescriptor::Tuple(tuple) => {
                for descriptor in &tuple.descriptors {
                    Self::collect_descriptor_direct_dependencies(descriptor, direct, dependencies);
                }
            }

            RsDescriptor::Enum(r#enum) => {
                for variant in &r#enum.variants {
                    if let Some(RsEnumVariantDescriptor::Descriptor(descriptor)) =
                        &variant.descriptor
                    {
                        Self::collect_descriptor_direct_dependencies(
                            descriptor,
                            direct,
                            dependencies,
                        );
                    }
                }
            }

            RsDescriptor::Vec(array) => {
                Self::collect_descriptor_direct_dependencies(
                    &array.descriptor,
                    false,
                    dependencies,
                );
            }

            RsDescriptor::Map(map) => {
                Self::collect_descriptor_direct_dependencies(&map.key, false, dependencies);
                Self::collect_descriptor_direct_dependencies(&map.descriptor, false, dependencies);
            }

            RsDescriptor::Box(inner) => {
                Self::collect_descriptor_direct_dependencies(inner, false, dependencies);
            }

            RsDescriptor::Primitive(_) | RsDescriptor::InlineUse(_) | RsDescriptor::Any(_) => {}
        }
    }
}
