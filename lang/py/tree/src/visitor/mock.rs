use crate::prelude::internal::*;

#[derive(Debug, Clone, PartialEq)]
pub enum PYMockVisited {
    Alias(PYAlias),
    Any(PYAny),
    Class(PYClass),
    Definition(PYDefinition),
    Descriptor(PYDescriptor),
    Dependency(PYDependencyIdent),
    Dict(PYDict),
    DictKey(PYDictKey),
    Doc(PYDoc),
    Extension(PYExtension),
    Identifier(PYIdentifier),
    Import(PYImport),
    ImportName(PYImportName),
    ImportReference(PYImportReference),
    Key(PYKey),
    List(PYList),
    Literal(PYLiteral),
    Module(PYModule),
    Path(PYPath),
    Primitive(PYPrimitive),
    Property(PYProperty),
    Reference(PYReference),
    Tuple(PYTuple),
    Union(PYUnion),
}

pub struct PYMockVisitor {
    pub visited: Vec<PYMockVisited>,
}

impl PYMockVisitor {
    pub fn new() -> Self {
        Self {
            visited: Vec::new(),
        }
    }
}

impl PYVisitor for PYMockVisitor {
    fn visit_alias(&mut self, alias: &mut PYAlias) {
        self.visited.push(PYMockVisited::Alias(alias.clone()));
    }

    fn visit_any(&mut self, any: &mut PYAny) {
        self.visited.push(PYMockVisited::Any(any.clone()));
    }

    fn visit_class(&mut self, class: &mut PYClass) {
        self.visited.push(PYMockVisited::Class(class.clone()));
    }

    fn visit_definition(&mut self, definition: &mut PYDefinition) {
        self.visited
            .push(PYMockVisited::Definition(definition.clone()));
    }

    fn visit_dependency(&mut self, dependency: &mut PYDependencyIdent) {
        self.visited
            .push(PYMockVisited::Dependency(dependency.clone()));
    }

    fn visit_descriptor(&mut self, descriptor: &mut PYDescriptor) {
        self.visited
            .push(PYMockVisited::Descriptor(descriptor.clone()));
    }

    fn visit_dict(&mut self, dict: &mut PYDict) {
        self.visited.push(PYMockVisited::Dict(dict.clone()));
    }

    fn visit_dict_key(&mut self, dict_key: &mut PYDictKey) {
        self.visited.push(PYMockVisited::DictKey(dict_key.clone()));
    }

    fn visit_doc(&mut self, doc: &mut PYDoc) {
        self.visited.push(PYMockVisited::Doc(doc.clone()));
    }

    fn visit_extension(&mut self, extension: &mut PYExtension) {
        self.visited
            .push(PYMockVisited::Extension(extension.clone()));
    }

    fn visit_identifier(&mut self, identifier: &mut PYIdentifier) {
        self.visited
            .push(PYMockVisited::Identifier(identifier.clone()));
    }

    fn visit_import(&mut self, import: &mut PYImport) {
        self.visited.push(PYMockVisited::Import(import.clone()));
    }

    fn visit_import_name(&mut self, import_name: &mut PYImportName) {
        self.visited
            .push(PYMockVisited::ImportName(import_name.clone()));
    }

    fn visit_import_reference(&mut self, import_reference: &mut PYImportReference) {
        self.visited
            .push(PYMockVisited::ImportReference(import_reference.clone()));
    }

    fn visit_key(&mut self, key: &mut PYKey) {
        self.visited.push(PYMockVisited::Key(key.clone()));
    }

    fn visit_list(&mut self, list: &mut PYList) {
        self.visited.push(PYMockVisited::List(list.clone()));
    }

    fn visit_literal(&mut self, literal: &mut PYLiteral) {
        self.visited.push(PYMockVisited::Literal(literal.clone()));
    }

    fn visit_module(&mut self, module: &mut PYModule) {
        self.visited.push(PYMockVisited::Module(module.clone()));
    }

    fn visit_path(&mut self, path: &mut PYPath) {
        self.visited.push(PYMockVisited::Path(path.clone()));
    }

    fn visit_primitive(&mut self, primitive: &mut PYPrimitive) {
        self.visited
            .push(PYMockVisited::Primitive(primitive.clone()));
    }

    fn visit_property(&mut self, property: &mut PYProperty) {
        self.visited.push(PYMockVisited::Property(property.clone()));
    }

    fn visit_reference(&mut self, reference: &mut PYReference) {
        self.visited
            .push(PYMockVisited::Reference(reference.clone()));
    }

    fn visit_tuple(&mut self, tuple: &mut PYTuple) {
        self.visited.push(PYMockVisited::Tuple(tuple.clone()));
    }

    fn visit_union(&mut self, union: &mut PYUnion) {
        self.visited.push(PYMockVisited::Union(union.clone()));
    }
}
