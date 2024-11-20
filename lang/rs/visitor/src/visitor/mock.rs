use genotype_lang_rs_tree::*;

use crate::visitor::RSVisitor;

#[derive(Debug, Clone, PartialEq)]
pub enum RSMockVisited {
    Alias(RSAlias),
    Any(RSAny),
    Attribute(RSAttribute),
    Definition(RSDefinition),
    Dependency(RSDependency),
    Descriptor(RSDescriptor),
    Doc(RSDoc),
    Enum(RSEnum),
    EnumVariant(RSEnumVariant),
    EnumVariantDescriptor(RSEnumVariantDescriptor),
    Field(RSField),
    FieldName(RSFieldName),
    HashMap(RSMap),
    Identifier(RSIdentifier),
    InlineUse(RSInlineUse),
    Module(RSModule),
    Option(RSOption),
    Path(RSPath),
    Primitive(RSPrimitive),
    Reference(RSReference),
    Struct(RSStruct),
    StructFields(RSStructFields),
    Tuple(RSTuple),
    Use(RSUse),
    UseName(RSUseName),
    UseReference(RSUseReference),
    Vec(RSVec),
}

pub struct RSMockVisitor {
    pub visited: Vec<RSMockVisited>,
}

impl RSMockVisitor {
    pub fn new() -> Self {
        Self {
            visited: Vec::new(),
        }
    }
}

impl RSVisitor for RSMockVisitor {
    fn visit_alias(&mut self, alias: &mut RSAlias) {
        self.visited.push(RSMockVisited::Alias(alias.clone()));
    }

    fn visit_any(&mut self, any: &mut RSAny) {
        self.visited.push(RSMockVisited::Any(any.clone()));
    }

    fn visit_attribute(&mut self, attribute: &mut RSAttribute) {
        self.visited
            .push(RSMockVisited::Attribute(attribute.clone()));
    }

    fn visit_definition(&mut self, definition: &mut RSDefinition) {
        self.visited
            .push(RSMockVisited::Definition(definition.clone()));
    }

    fn visit_dependency(&mut self, dependency: &mut RSDependency) {
        self.visited
            .push(RSMockVisited::Dependency(dependency.clone()));
    }

    fn visit_descriptor(&mut self, descriptor: &mut RSDescriptor) {
        self.visited
            .push(RSMockVisited::Descriptor(descriptor.clone()));
    }

    fn visit_doc(&mut self, doc: &mut RSDoc) {
        self.visited.push(RSMockVisited::Doc(doc.clone()));
    }

    fn visit_enum(&mut self, r#enum: &mut RSEnum) {
        self.visited.push(RSMockVisited::Enum(r#enum.clone()));
    }

    fn visit_enum_variant(&mut self, variant: &mut RSEnumVariant) {
        self.visited
            .push(RSMockVisited::EnumVariant(variant.clone()));
    }

    fn visit_enum_variant_descriptor(&mut self, descriptor: &mut RSEnumVariantDescriptor) {
        self.visited
            .push(RSMockVisited::EnumVariantDescriptor(descriptor.clone()));
    }

    fn visit_field(&mut self, field: &mut RSField) {
        self.visited.push(RSMockVisited::Field(field.clone()));
    }

    fn visit_field_name(&mut self, field_name: &mut RSFieldName) {
        self.visited
            .push(RSMockVisited::FieldName(field_name.clone()));
    }

    fn visit_map(&mut self, map: &mut RSMap) {
        self.visited.push(RSMockVisited::HashMap(map.clone()));
    }

    fn visit_identifier(&mut self, identifier: &mut RSIdentifier) {
        self.visited
            .push(RSMockVisited::Identifier(identifier.clone()));
    }

    fn visit_inline_use(&mut self, inline_use: &mut RSInlineUse) {
        self.visited
            .push(RSMockVisited::InlineUse(inline_use.clone()));
    }

    fn visit_module(&mut self, module: &mut RSModule) {
        self.visited.push(RSMockVisited::Module(module.clone()));
    }

    fn visit_option(&mut self, option: &mut RSOption) {
        self.visited.push(RSMockVisited::Option(option.clone()));
    }

    fn visit_path(&mut self, path: &mut RSPath) {
        self.visited.push(RSMockVisited::Path(path.clone()));
    }

    fn visit_primitive(&mut self, primitive: &mut RSPrimitive) {
        self.visited
            .push(RSMockVisited::Primitive(primitive.clone()));
    }

    fn visit_reference(&mut self, reference: &mut RSReference) {
        self.visited
            .push(RSMockVisited::Reference(reference.clone()));
    }

    fn visit_struct(&mut self, r#struct: &mut RSStruct) {
        self.visited.push(RSMockVisited::Struct(r#struct.clone()));
    }

    fn visit_struct_fields(&mut self, fields: &mut RSStructFields) {
        self.visited
            .push(RSMockVisited::StructFields(fields.clone()));
    }

    fn visit_tuple(&mut self, tuple: &mut RSTuple) {
        self.visited.push(RSMockVisited::Tuple(tuple.clone()));
    }

    fn visit_use(&mut self, r#use: &mut RSUse) {
        self.visited.push(RSMockVisited::Use(r#use.clone()));
    }

    fn visit_use_name(&mut self, use_name: &mut RSUseName) {
        self.visited.push(RSMockVisited::UseName(use_name.clone()));
    }

    fn visit_use_reference(&mut self, reference: &mut RSUseReference) {
        self.visited
            .push(RSMockVisited::UseReference(reference.clone()));
    }

    fn visit_vec(&mut self, vec: &mut RSVec) {
        self.visited.push(RSMockVisited::Vec(vec.clone()));
    }
}
