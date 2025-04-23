use genotype_lang_py_tree::*;

use crate::visitor::PYVisitor;

use super::PYTraverse;

impl PYTraverse for PYModule {
    fn traverse(&mut self, visitor: &mut dyn PYVisitor) {
        visitor.visit_module(self);

        if let Some(doc) = &mut self.doc {
            doc.traverse(visitor);
        }

        for import in &mut self.imports {
            import.traverse(visitor);
        }

        for definitions in &mut self.definitions {
            definitions.traverse(visitor);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visitor::mock::*;
    use genotype_lang_py_tree::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_traverse_base() {
        let mut visitor = PYMockVisitor::new();
        let path = PYPath("./path/to/module".into());
        let reference = PYImportReference::Glob;
        let dependency = PYDependencyIdent::Local(path.clone());
        let import = PYImport {
            reference: reference.clone(),
            dependency: dependency.clone(),
        };
        let ref_identifier = PYIdentifier("Reference".into());
        let alias = PYAlias {
            doc: None,
            name: PYIdentifier("Name".into()),
            descriptor: PYPrimitive::String.into(),
            references: vec![ref_identifier.clone()],
        };
        let definition = PYDefinition::Alias(alias.clone());
        let mut module = PYModule {
            doc: None,
            imports: vec![import.clone()],
            definitions: vec![definition.clone()],
        };
        module.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                PYMockVisited::Module(module.clone()),
                PYMockVisited::Import(import.clone()),
                PYMockVisited::ImportReference(reference),
                PYMockVisited::Dependency(dependency),
                PYMockVisited::Path(path.clone()),
                PYMockVisited::Definition(definition.clone()),
                PYMockVisited::Alias(alias.clone()),
                PYMockVisited::Descriptor(alias.descriptor.clone()),
                PYMockVisited::Primitive(PYPrimitive::String),
                PYMockVisited::Identifier(ref_identifier)
            ]
        );
    }

    #[test]
    fn test_traverse_doc() {
        let mut visitor = PYMockVisitor::new();
        let path = PYPath("./path/to/module".into());
        let reference = PYImportReference::Glob;
        let dependency = PYDependencyIdent::Local(path.clone());
        let import = PYImport {
            reference: reference.clone(),
            dependency: dependency.clone(),
        };
        let ref_identifier = PYIdentifier("Reference".into());
        let alias = PYAlias {
            doc: None,
            name: PYIdentifier("Name".into()),
            descriptor: PYPrimitive::String.into(),
            references: vec![ref_identifier.clone()],
        };
        let definition = PYDefinition::Alias(alias.clone());
        let mut module = PYModule {
            doc: Some(PYDoc("Hello, world!".into())),
            imports: vec![import.clone()],
            definitions: vec![definition.clone()],
        };
        module.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                PYMockVisited::Module(module.clone()),
                PYMockVisited::Doc(module.doc.clone().unwrap()),
                PYMockVisited::Import(import.clone()),
                PYMockVisited::ImportReference(reference.clone()),
                PYMockVisited::Dependency(dependency),
                PYMockVisited::Path(path),
                PYMockVisited::Definition(definition.clone()),
                PYMockVisited::Alias(alias.clone()),
                PYMockVisited::Descriptor(alias.descriptor.clone()),
                PYMockVisited::Primitive(PYPrimitive::String),
                PYMockVisited::Identifier(ref_identifier)
            ]
        );
    }
}
