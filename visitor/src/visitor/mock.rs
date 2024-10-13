use genotype_parser::tree::*;

use crate::visitor::GTVisitor;

#[derive(Debug, Clone, PartialEq)]
pub enum GTMockVisited {
    Alias(GTAlias),
    Array(GTArray),
    Descriptor(GTDescriptor),
    Doc(GTDoc),
    Extension(GTExtension),
    Identifier(GTIdentifier),
    Import(GTImport),
    ImportName(GTImportName),
    ImportReference(GTImportReference),
    InlineImport(GTInlineImport),
    Literal(GTLiteral),
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
    fn visit_alias(&mut self, alias: &mut GTAlias) {
        self.visited.push(GTMockVisited::Alias(alias.clone()));
    }

    fn visit_array(&mut self, array: &mut GTArray) {
        self.visited.push(GTMockVisited::Array(array.clone()));
    }

    fn visit_descriptor(&mut self, descriptor: &mut GTDescriptor) {
        self.visited
            .push(GTMockVisited::Descriptor(descriptor.clone()));
    }

    fn visit_doc(&mut self, doc: &mut GTDoc) {
        self.visited.push(GTMockVisited::Doc(doc.clone()));
    }

    fn visit_extension(&mut self, extension: &mut GTExtension) {
        self.visited
            .push(GTMockVisited::Extension(extension.clone()));
    }

    fn visit_identifier(&mut self, identifier: &mut GTIdentifier) {
        self.visited
            .push(GTMockVisited::Identifier(identifier.clone()));
    }

    fn visit_import(&mut self, import: &mut GTImport) {
        self.visited.push(GTMockVisited::Import(import.clone()));
    }

    fn visit_import_name(&mut self, import_name: &mut GTImportName) {
        self.visited
            .push(GTMockVisited::ImportName(import_name.clone()));
    }

    fn visit_import_reference(&mut self, import_reference: &mut GTImportReference) {
        self.visited
            .push(GTMockVisited::ImportReference(import_reference.clone()));
    }

    fn visit_inline_import(&mut self, inline_import: &mut GTInlineImport) {
        self.visited
            .push(GTMockVisited::InlineImport(inline_import.clone()));
    }

    fn visit_literal(&mut self, literal: &mut GTLiteral) {
        self.visited.push(GTMockVisited::Literal(literal.clone()));
    }

    fn visit_key(&mut self, key: &mut GTKey) {
        self.visited.push(GTMockVisited::Key(key.clone()));
    }

    fn visit_module(&mut self, module: &mut GTModule) {
        self.visited.push(GTMockVisited::Module(module.clone()));
    }

    fn visit_object(&mut self, object: &mut GTObject) {
        self.visited.push(GTMockVisited::Object(object.clone()));
    }

    fn visit_path(&mut self, path: &mut GTPath) {
        self.visited.push(GTMockVisited::Path(path.clone()));
    }

    fn visit_primitive(&mut self, primitive: &mut GTPrimitive) {
        self.visited
            .push(GTMockVisited::Primitive(primitive.clone()));
    }

    fn visit_property(&mut self, property: &mut GTProperty) {
        self.visited.push(GTMockVisited::Property(property.clone()));
    }

    fn visit_reference(&mut self, reference: &mut GTReference) {
        self.visited
            .push(GTMockVisited::Reference(reference.clone()));
    }

    fn visit_tuple(&mut self, tuple: &mut GTTuple) {
        self.visited.push(GTMockVisited::Tuple(tuple.clone()));
    }

    fn visit_union(&mut self, union: &mut GTUnion) {
        self.visited.push(GTMockVisited::Union(union.clone()));
    }
}
