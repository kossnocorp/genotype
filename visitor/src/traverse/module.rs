use genotype_parser::tree::module::GTModule;

use crate::visitor::GTVisitor;

use super::GTTraverse;

impl GTTraverse for GTModule {
    fn traverse(&mut self, visitor: &mut dyn GTVisitor) {
        visitor.visit_module(self);

        if let Some(doc) = &mut self.doc {
            doc.traverse(visitor);
        }

        for import in &mut self.imports {
            import.traverse(visitor);
        }

        for alias in &mut self.aliases {
            alias.traverse(visitor);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::visitor::mock::*;
    use genotype_parser::tree::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_traverse_base() {
        let mut visitor = GTMockVisitor::new();
        let import_path = GTPath::new("./path/to/module".into());
        let reference = GTImportReference::Glob;
        let import = GTImport {
            path: import_path.clone(),
            reference: reference.clone(),
        };
        let alias = GTAlias {
            doc: None,
            name: "Name".into(),
            descriptor: GTPrimitive::String.into(),
        };
        let mut module = GTModule {
            doc: None,
            imports: vec![import.clone()],
            aliases: vec![alias.clone()],
        };
        module.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::Module(module.clone()),
                GTMockVisited::Import(import.clone()),
                GTMockVisited::Path(import_path.clone()),
                GTMockVisited::ImportReference(reference.clone()),
                GTMockVisited::Alias(alias.clone()),
                GTMockVisited::Descriptor(alias.descriptor.clone()),
                GTMockVisited::Primitive(GTPrimitive::String),
            ]
        );
    }

    #[test]
    fn test_traverse_doc() {
        let mut visitor = GTMockVisitor::new();
        let import_path = GTPath::new("./path/to/import".into());
        let reference = GTImportReference::Glob;
        let import = GTImport {
            path: import_path.clone(),
            reference: reference.clone(),
        };
        let alias = GTAlias {
            doc: None,
            name: "Name".into(),
            descriptor: GTPrimitive::String.into(),
        };
        let mut module = GTModule {
            doc: Some(GTDoc("Hello, world!".into())),
            imports: vec![import.clone()],
            aliases: vec![alias.clone()],
        };
        module.traverse(&mut visitor);
        assert_eq!(
            visitor.visited,
            vec![
                GTMockVisited::Module(module.clone()),
                GTMockVisited::Doc(module.doc.clone().unwrap()),
                GTMockVisited::Import(import.clone()),
                GTMockVisited::Path(import_path.clone()),
                GTMockVisited::ImportReference(reference.clone()),
                GTMockVisited::Alias(alias.clone()),
                GTMockVisited::Descriptor(alias.descriptor.clone()),
                GTMockVisited::Primitive(GTPrimitive::String),
            ]
        );
    }
}
