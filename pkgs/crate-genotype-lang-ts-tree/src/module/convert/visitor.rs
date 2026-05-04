use crate::prelude::internal::*;

pub struct TsModuleConvertVisitor {
    definitions: IndexSet<TsIdentifier>,
    scope: IndexSet<TsIdentifier>,
    current_definition: Option<TsIdentifier>,
}

impl TsModuleConvertVisitor {
    pub fn new(module: &TsModule) -> Self {
        let definitions = module
            .definitions
            .iter()
            .map(|definition| definition.name())
            .collect::<IndexSet<TsIdentifier>>();

        Self {
            definitions,
            scope: Default::default(),
            current_definition: None,
        }
    }
}

impl TsVisitor for TsModuleConvertVisitor {}

impl TsVisitorMut for TsModuleConvertVisitor {
    fn visit_definition_mut(&mut self, definition: &mut TsDefinition) {
        let name = definition.name();
        self.scope.insert(name.clone());
        self.current_definition = Some(name);
    }

    fn visit_reference_mut(&mut self, reference: &mut TsReference) {
        if !self.definitions.contains(&reference.identifier) {
            return;
        }

        if self.current_definition.as_ref() == Some(&reference.identifier) {
            reference.rel = TsReferenceRel::SelfRecursive;
            return;
        }

        reference.rel = if !self.scope.contains(&reference.identifier) {
            TsReferenceRel::Forward
        } else {
            TsReferenceRel::Regular
        };
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;
    use crate::test::*;

    #[test]
    fn test_preserves_regular_references() {
        let mut module = Tst::module(
            vec![],
            vec_into![
                Tst::alias("Bar", Tst::primitive_string()),
                Tst::alias("Foo", Tst::reference("Bar")),
            ],
        );

        let mut visitor = TsModuleConvertVisitor::new(&module);
        module.traverse_mut(&mut visitor);

        assert_ron_snapshot!(module, @r#"
        TsModule(
          doc: None,
          imports: [],
          definitions: [
            Alias(TsAlias(
              doc: None,
              name: TsIdentifier("Bar"),
              generics: [],
              descriptor: Primitive(String),
            )),
            Alias(TsAlias(
              doc: None,
              name: TsIdentifier("Foo"),
              generics: [],
              descriptor: Reference(TsReference(
                identifier: TsIdentifier("Bar"),
                arguments: [],
                rel: Regular,
              )),
            )),
          ],
        )
        "#);
    }

    #[test]
    fn test_marks_forward_references() {
        let mut module = Tst::module(
            vec![],
            vec_into![
                Tst::alias("Foo", Tst::reference("Bar")),
                Tst::alias("Bar", Tst::primitive_string()),
            ],
        );

        let mut visitor = TsModuleConvertVisitor::new(&module);
        module.traverse_mut(&mut visitor);

        assert_ron_snapshot!(module, @r#"
        TsModule(
          doc: None,
          imports: [],
          definitions: [
            Alias(TsAlias(
              doc: None,
              name: TsIdentifier("Foo"),
              generics: [],
              descriptor: Reference(TsReference(
                identifier: TsIdentifier("Bar"),
                arguments: [],
                rel: Forward,
              )),
            )),
            Alias(TsAlias(
              doc: None,
              name: TsIdentifier("Bar"),
              generics: [],
              descriptor: Primitive(String),
            )),
          ],
        )
        "#);
    }

    #[test]
    fn test_marks_self_self_recursive_references() {
        let mut module = Tst::module(
            vec![],
            vec_into![
                Tst::alias("Alias", Tst::reference("Alias")),
                Tst::alias("Array", Tst::array(Tst::reference("Array"))),
                Tst::alias(
                    "Tuple",
                    Tst::tuple(vec_into![Tst::literal_null(), Tst::reference("Tuple")]),
                ),
                Tst::alias(
                    "Object",
                    Tst::object(vec![Tst::property("self", Tst::reference("Object"))])
                ),
            ],
        );

        let mut visitor = TsModuleConvertVisitor::new(&module);
        module.traverse_mut(&mut visitor);

        assert_ron_snapshot!(module, @r#"
        TsModule(
          doc: None,
          imports: [],
          definitions: [
            Alias(TsAlias(
              doc: None,
              name: TsIdentifier("Alias"),
              generics: [],
              descriptor: Reference(TsReference(
                identifier: TsIdentifier("Alias"),
                arguments: [],
                rel: SelfRecursive,
              )),
            )),
            Alias(TsAlias(
              doc: None,
              name: TsIdentifier("Array"),
              generics: [],
              descriptor: Array(TsArray(
                descriptor: Reference(TsReference(
                  identifier: TsIdentifier("Array"),
                  arguments: [],
                  rel: SelfRecursive,
                )),
              )),
            )),
            Alias(TsAlias(
              doc: None,
              name: TsIdentifier("Tuple"),
              generics: [],
              descriptor: Tuple(TsTuple(
                descriptors: [
                  Literal(Null),
                  Reference(TsReference(
                    identifier: TsIdentifier("Tuple"),
                    arguments: [],
                    rel: SelfRecursive,
                  )),
                ],
              )),
            )),
            Alias(TsAlias(
              doc: None,
              name: TsIdentifier("Object"),
              generics: [],
              descriptor: Object(TsObject(
                properties: [
                  TsProperty(
                    doc: None,
                    name: TsKey("self"),
                    descriptor: Reference(TsReference(
                      identifier: TsIdentifier("Object"),
                      arguments: [],
                      rel: SelfRecursive,
                    )),
                    required: true,
                  ),
                ],
              )),
            )),
          ],
        )
        "#);
    }
}
