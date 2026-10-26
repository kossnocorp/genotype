use crate::prelude::internal::*;

pub(crate) struct TsEffectResolver {
    recursive_names: IndexSet<TsIdentifier>,
    pub(crate) resolution: TsEffectResolution,
}

impl TsEffectResolver {
    pub(crate) fn new(recursive_names: IndexSet<TsIdentifier>) -> Self {
        Self {
            recursive_names,
            resolution: TsEffectResolution::default(),
        }
    }
}

impl TsVisitor for TsEffectResolver {
    #[cfg(test)]
    fn visit_module(&mut self, module: &TsModule) {
        self.recursive_names = TsModule::resolve_definition_order(module.definitions.clone()).1;
    }

    fn visit_alias(&mut self, alias: &TsAlias) {
        let refs = alias.descriptor.scan_references();
        if refs.has_self_recursive || refs.has_forward {
            self.resolution.aliases.insert(
                alias.name.clone(),
                TsEffectAlias {
                    generic: (!alias.generics.is_empty() && refs.has_self_recursive)
                        .then(|| TsEffectGeneric::resolve(&alias.descriptor, &alias.generics)),
                },
            );
        }
    }

    fn visit_interface(&mut self, interface: &TsInterface) {
        let scans = interface
            .properties
            .iter()
            .map(|p| p.descriptor.scan_references())
            .collect::<Vec<_>>();
        let has_forward = scans.iter().any(|refs| refs.has_forward);
        let recursive_properties = has_forward || scans.iter().any(|refs| refs.has_self_recursive);
        if !recursive_properties && !self.recursive_names.contains(&interface.name) {
            return;
        }
        if interface.generics.is_empty() {
            let mut native = interface.clone();
            if !native.properties.is_empty() {
                let keys: TsDescriptor = TsUnion {
                    descriptors: native
                        .properties
                        .iter()
                        .map(|p| TsLiteral::String(p.name.0.to_string()).into())
                        .collect(),
                }
                .into();
                for extension in &mut native.extensions {
                    extension.reference = TsReference::new(
                        "Omit".into(),
                        vec![extension.reference.clone().into(), keys.clone()],
                        TsReferenceRel::Regular,
                    );
                }
            }
            self.resolution.interfaces.insert(
                interface.name.clone(),
                TsEffectInterface::Recursive {
                    native: Box::new(native),
                    annotate_fields: has_forward,
                },
            );
        } else if recursive_properties {
            let object = TsObject {
                properties: interface.properties.clone(),
            }
            .into();
            self.resolution.interfaces.insert(
                interface.name.clone(),
                TsEffectInterface::Generic {
                    definition: Box::new(TsEffectGeneric::resolve(&object, &interface.generics)),
                    properties: interface
                        .properties
                        .iter()
                        .map(|p| TsEffectGeneric::resolve(&p.descriptor, &interface.generics))
                        .collect(),
                },
            );
        }
    }

    fn visit_property(&mut self, property: &TsProperty) {
        let refs = property.descriptor.scan_references();
        if (refs.has_forward || refs.has_self_recursive)
            && !self
                .resolution
                .suspended_descriptors
                .contains(&property.descriptor)
        {
            self.resolution
                .suspended_descriptors
                .push(property.descriptor.clone());
        }
    }
}

impl TsEffectGeneric {
    pub(crate) fn resolve(descriptor: &TsDescriptor, generics: &[TsIdentifier]) -> Self {
        let source = descriptor.clone();
        let mut structural = source.clone();
        structural.traverse_mut(&mut EffectRecursiveTypeVisitor {
            projection: None,
            generics,
        });
        let mut decoded = source.clone();
        decoded.traverse_mut(&mut EffectRecursiveTypeVisitor {
            projection: Some("Type"),
            generics,
        });
        let mut encoded = source.clone();
        encoded.traverse_mut(&mut EffectRecursiveTypeVisitor {
            projection: Some("Encoded"),
            generics,
        });
        let deferred = source;
        Self {
            structural,
            codec: TsEffectCodec { decoded, encoded },
            deferred,
        }
    }
}

// Keep schema-parameter aliases public while instantiating structural recursion
// separately for decoded and encoded values.
struct EffectRecursiveTypeVisitor<'a> {
    projection: Option<&'static str>,
    generics: &'a [TsIdentifier],
}
impl TsVisitor for EffectRecursiveTypeVisitor<'_> {}
impl TsVisitorMut for EffectRecursiveTypeVisitor<'_> {
    fn visit_reference_mut(&mut self, reference: &mut TsReference) {
        if matches!(reference.rel, TsReferenceRel::SelfRecursive) {
            reference.identifier = format!("{}.Type", reference.identifier.0).into();
        } else if let Some(projection) = self.projection
            && self.generics.contains(&reference.identifier)
        {
            reference.identifier = format!("{}[\"{projection}\"]", reference.identifier.0).into();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn recursive_reference() -> TsReference {
        TsReference::new(
            "Node".into(),
            vec![Tst::reference("T").into()],
            TsReferenceRel::SelfRecursive,
        )
    }

    #[test]
    fn test_prepare_generic_sides_and_deferred_properties() {
        let descriptor = Tst::object(vec![
            Tst::property("value", Tst::reference("T")),
            Tst::property_optional("next", recursive_reference()),
        ])
        .into();
        let prepared = TsEffectGeneric::resolve(&descriptor, &[Tst::identifier("T")]);
        assert_snapshot!(render_node(prepared.structural), @"
        {
          value: T,
          next?: Node.Type<T>
        }
        ");
        assert_snapshot!(render_node(prepared.codec.decoded), @r#"
        {
          value: T["Type"],
          next?: Node.Type<T["Type"]>
        }
        "#);
        assert_snapshot!(render_node(prepared.codec.encoded), @r#"
        {
          value: T["Encoded"],
          next?: Node.Type<T["Encoded"]>
        }
        "#);
        let TsDescriptor::Object(deferred) = prepared.deferred else {
            panic!("object descriptor");
        };
        assert!(
            deferred.properties[1]
                .descriptor
                .scan_references()
                .has_self_recursive
        );
    }

    #[test]
    fn test_prepare_cyclic_extension_and_ordinary_dependent() {
        let cyclic = TsInterface {
            extensions: vec![Tst::extension("Base")],
            ..Tst::interface(
                "Node",
                vec![Tst::property("kind", Tst::literal_string("node"))],
            )
        };
        let dependent = TsInterface {
            extensions: vec![Tst::extension("Node")],
            ..Tst::interface("Dependent", vec![])
        };
        let mut resolver = TsEffectResolver::new(IndexSet::from([Tst::identifier("Node")]));
        cyclic.traverse(&mut resolver);
        dependent.traverse(&mut resolver);
        let Some(TsEffectInterface::Recursive {
            native,
            annotate_fields,
        }) = resolver.resolution.interfaces.get(&cyclic.name)
        else {
            panic!("recursive interface");
        };
        assert!(!annotate_fields);
        assert_snapshot!(render_node(native.as_ref().clone()), @r#"
        export interface Node extends Omit<Base, "kind"> {
          kind: "node";
        }
        "#);
        assert!(!resolver.resolution.interfaces.contains_key(&dependent.name));
    }

    #[test]
    fn test_prepare_forward_property_and_resolver_is_idempotent() {
        let node = Tst::interface(
            "Node",
            vec![Tst::property("next", Tst::reference_forward("Later"))],
        );
        let mut resolver = TsEffectResolver::new(IndexSet::new());
        node.traverse(&mut resolver);
        assert!(
            resolver
                .resolution
                .suspended_descriptors
                .contains(&node.properties[0].descriptor)
        );
        assert!(matches!(
            resolver.resolution.interfaces.get(&node.name),
            Some(TsEffectInterface::Recursive {
                annotate_fields: true,
                ..
            })
        ));
        let resolved = resolver.resolution.clone();
        node.traverse(&mut resolver);
        assert_eq!(resolver.resolution, resolved);
    }

    #[test]
    fn test_prepare_generic_alias_and_interface() {
        let alias = Tst::alias_with_generics("Node", vec!["T"], Tst::array(recursive_reference()));
        let interface = Tst::interface_with_generics(
            "Node",
            vec!["T"],
            vec![Tst::property("next", recursive_reference())],
        );
        let mut resolver = TsEffectResolver::new(IndexSet::new());
        alias.traverse(&mut resolver);
        interface.traverse(&mut resolver);
        assert!(resolver.resolution.aliases[&alias.name].generic.is_some());
        assert!(
            matches!(resolver.resolution.interfaces.get(&interface.name), Some(TsEffectInterface::Generic { properties, .. }) if properties.len() == 1)
        );
        let resolved = resolver.resolution.clone();
        alias.traverse(&mut resolver);
        interface.traverse(&mut resolver);
        assert_eq!(resolver.resolution, resolved);
    }
}
