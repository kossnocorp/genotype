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
    use miette::NamedSource;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_traverse_base() {
        let mut visitor = GTMockVisitor::new();
        let import_path = GTPath::parse((0, 0).into(), "./path/to/module").unwrap();
        let reference = GTImportReference::Glob((0, 0).into());
        let import = GTImport {
            span: (0, 0).into(),
            path: import_path.clone(),
            reference: reference.clone(),
        };
        let alias = GTAlias {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            name: GTIdentifier::new((0, 0).into(), "Name".into()),
            descriptor: GTPrimitive::String((0, 0).into()).into(),
        };
        let mut module = GTModule {
            id: "module".into(),
            source_code: NamedSource::new("module.type", "".into()),
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
                GTMockVisited::Primitive(GTPrimitive::String((0, 0).into())),
            ]
        );
    }

    #[test]
    fn test_traverse_doc() {
        let mut visitor = GTMockVisitor::new();
        let import_path = GTPath::parse((0, 0).into(), "./path/to/import").unwrap();
        let reference = GTImportReference::Glob((0, 0).into());
        let import = GTImport {
            span: (0, 0).into(),
            path: import_path.clone(),
            reference: reference.clone(),
        };
        let alias = GTAlias {
            span: (0, 0).into(),
            doc: None,
            attributes: vec![],
            name: GTIdentifier::new((0, 0).into(), "Name".into()),
            descriptor: GTPrimitive::String((0, 0).into()).into(),
        };
        let mut module = GTModule {
            id: "module".into(),
            source_code: NamedSource::new("module.type", "".into()),
            doc: Some(GTDoc::new((0, 0).into(), "Hello, world!".into())),
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
                GTMockVisited::Primitive(GTPrimitive::String((0, 0).into())),
            ]
        );
    }
}
