use genotype_parser::tree::*;

use crate::visitor::GTVisitor;

#[derive(Debug, Clone, PartialEq)]
pub enum GTMockVisited {
    Alias(GTAlias),
    Array(GTArray),
    Descriptor(GTDescriptor),
    Doc(GTDoc),
    Identifier(GTIdentifier),
    Import(GTImport),
    ImportName(GTImportName),
    ImportReference(GTImportReference),
    InlineImport(GTInlineImport),
    Key(GTKey),
    Module(GTModule),
    Object(GTObject),
    Path(GTPath),
    Primitive(GTPrimitive),
    Property(GTProperty),
    Reference(GTReference),
    Tuple(GTTuple),
    Union(GTUnion),
}

pub struct GTMockVisitor {
    pub visited: Vec<GTMockVisited>,
}

impl GTMockVisitor {
    pub fn new() -> Self {
        Self {
            visited: Vec::new(),
        }
    }
}

impl GTVisitor for GTMockVisitor {
    fn visit_alias(&mut self, alias: &GTAlias) {
        self.visited.push(GTMockVisited::Alias(alias.clone()));
    }

    fn visit_array(&mut self, array: &GTArray) {
        self.visited.push(GTMockVisited::Array(array.clone()));
    }

    fn visit_descriptor(&mut self, descriptor: &GTDescriptor) {
        self.visited
            .push(GTMockVisited::Descriptor(descriptor.clone()));
    }

    fn visit_doc(&mut self, doc: &GTDoc) {
        self.visited.push(GTMockVisited::Doc(doc.clone()));
    }

    fn visit_identifier(&mut self, identifier: &GTIdentifier) {
        self.visited
            .push(GTMockVisited::Identifier(identifier.clone()));
    }

    fn visit_import(&mut self, import: &GTImport) {
        self.visited.push(GTMockVisited::Import(import.clone()));
    }

    fn visit_import_name(&mut self, import_name: &GTImportName) {
        self.visited
            .push(GTMockVisited::ImportName(import_name.clone()));
    }

    fn visit_import_reference(&mut self, import_reference: &GTImportReference) {
        self.visited
            .push(GTMockVisited::ImportReference(import_reference.clone()));
    }

    fn visit_inline_import(&mut self, inline_import: &GTInlineImport) {
        self.visited
            .push(GTMockVisited::InlineImport(inline_import.clone()));
    }

    fn visit_key(&mut self, key: &GTKey) {
        self.visited.push(GTMockVisited::Key(key.clone()));
    }

    fn visit_module(&mut self, module: &GTModule) {
        self.visited.push(GTMockVisited::Module(module.clone()));
    }

    fn visit_object(&mut self, object: &GTObject) {
        self.visited.push(GTMockVisited::Object(object.clone()));
    }

    fn visit_path(&mut self, path: &GTPath) {
        self.visited.push(GTMockVisited::Path(path.clone()));
    }

    fn visit_primitive(&mut self, primitive: &GTPrimitive) {
        self.visited
            .push(GTMockVisited::Primitive(primitive.clone()));
    }

    fn visit_property(&mut self, property: &GTProperty) {
        self.visited.push(GTMockVisited::Property(property.clone()));
    }

    fn visit_reference(&mut self, reference: &GTReference) {
        self.visited
            .push(GTMockVisited::Reference(reference.clone()));
    }

    fn visit_tuple(&mut self, tuple: &GTTuple) {
        self.visited.push(GTMockVisited::Tuple(tuple.clone()));
    }

    fn visit_union(&mut self, union: &GTUnion) {
        self.visited.push(GTMockVisited::Union(union.clone()));
    }
}