use crate::visitor::Visitor;
use genotype_parser::tree::{
    alias::Alias, array::Array, descriptor::Descriptor, import::Import, module::Module,
    object::Object, primitive::Primitive, property::Property, reference::Reference, tuple::Tuple,
};

pub fn traverse_module(module: &Module, visitor: &mut dyn Visitor) {
    visitor.visit_module(module);

    for import in &module.imports {
        traverse_import(&import, visitor);
    }

    for alias in &module.aliases {
        traverse_alias(alias, visitor);
    }
}

pub fn traverse_import(import: &Import, visitor: &mut dyn Visitor) {
    visitor.visit_import(import);
}

pub fn traverse_alias(alias: &Alias, visitor: &mut dyn Visitor) {
    visitor.visit_alias(alias);
    traverse_descriptor(&alias.descriptor, visitor);
}

pub fn traverse_descriptor(descriptor: &Descriptor, visitor: &mut dyn Visitor) {
    visitor.visit_descriptor(&descriptor);

    match descriptor {
        Descriptor::Alias(alias) => traverse_alias(alias, visitor),

        Descriptor::Object(object) => traverse_object(object, visitor),

        Descriptor::Primitive(primitive) => traverse_primitive(primitive, visitor),

        Descriptor::Array(array) => traverse_array(array, visitor),

        Descriptor::Name(name) => traverse_name(name, visitor),

        Descriptor::Tuple(tuple) => traverse_tuple(tuple, visitor),

        Descriptor::Reference(reference) => traverse_reference(reference, visitor),

        Descriptor::Nullable(descriptor) => traverse_descriptor(descriptor, visitor),
    }
}

pub fn traverse_object(object: &Object, visitor: &mut dyn Visitor) {
    visitor.visit_object(object);
    for property in &object.properties {
        traverse_property(property, visitor);
    }
}

pub fn traverse_property(property: &Property, visitor: &mut dyn Visitor) {
    visitor.visit_property(property);
    traverse_descriptor(&property.descriptor, visitor);
}

pub fn traverse_primitive(primitive: &Primitive, visitor: &mut dyn Visitor) {
    visitor.visit_primitive(primitive);
}

pub fn traverse_array(array: &Array, visitor: &mut dyn Visitor) {
    visitor.visit_array(array);
    traverse_descriptor(&array.descriptor, visitor);
}

pub fn traverse_name(name: &String, visitor: &mut dyn Visitor) {
    visitor.visit_name(name);
}

pub fn traverse_tuple(tuple: &Tuple, visitor: &mut dyn Visitor) {
    visitor.visit_tuple(tuple);
    for descriptor in &tuple.descriptors {
        traverse_descriptor(descriptor, visitor);
    }
}

pub fn traverse_reference(reference: &Reference, visitor: &mut dyn Visitor) {
    visitor.visit_reference(reference);
}
