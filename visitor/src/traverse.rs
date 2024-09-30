use crate::visitor::GTVisitor;
use genotype_parser::tree::{
    alias::GTAlias, array::GTArray, descriptor::GTDescriptor, import::GTImport, module::GTModule,
    object::GTObject, primitive::GTPrimitive, property::GTProperty, reference::GTReference,
    tuple::GTTuple,
};

pub fn traverse_gt_module(module: &GTModule, visitor: &mut dyn GTVisitor) {
    visitor.visit_module(module);

    for import in &module.imports {
        traverse_gt_import(&import, visitor);
    }

    for alias in &module.aliases {
        traverse_gt_alias(alias, visitor);
    }
}

pub fn traverse_gt_import(import: &GTImport, visitor: &mut dyn GTVisitor) {
    visitor.visit_import(import);
}

pub fn traverse_gt_alias(alias: &GTAlias, visitor: &mut dyn GTVisitor) {
    visitor.visit_alias(alias);
    traverse_gt_descriptor(&alias.descriptor, visitor);
}

pub fn traverse_gt_descriptor(descriptor: &GTDescriptor, visitor: &mut dyn GTVisitor) {
    visitor.visit_descriptor(&descriptor);

    match descriptor {
        GTDescriptor::Alias(alias) => traverse_gt_alias(alias, visitor),

        GTDescriptor::Object(object) => traverse_gt_object(object, visitor),

        GTDescriptor::Primitive(primitive) => traverse_gt_primitive(primitive, visitor),

        GTDescriptor::Array(array) => traverse_gt_array(array, visitor),

        GTDescriptor::Name(name) => traverse_gt_name(name, visitor),

        GTDescriptor::Tuple(tuple) => traverse_gt_tuple(tuple, visitor),

        GTDescriptor::Reference(reference) => traverse_gt_reference(reference, visitor),

        GTDescriptor::Nullable(descriptor) => traverse_gt_descriptor(descriptor, visitor),
    }
}

pub fn traverse_gt_object(object: &GTObject, visitor: &mut dyn GTVisitor) {
    visitor.visit_object(object);
    for property in &object.properties {
        traverse_gt_property(property, visitor);
    }
}

pub fn traverse_gt_property(property: &GTProperty, visitor: &mut dyn GTVisitor) {
    visitor.visit_property(property);
    traverse_gt_descriptor(&property.descriptor, visitor);
}

pub fn traverse_gt_primitive(primitive: &GTPrimitive, visitor: &mut dyn GTVisitor) {
    visitor.visit_primitive(primitive);
}

pub fn traverse_gt_array(array: &GTArray, visitor: &mut dyn GTVisitor) {
    visitor.visit_array(array);
    traverse_gt_descriptor(&array.descriptor, visitor);
}

pub fn traverse_gt_name(name: &String, visitor: &mut dyn GTVisitor) {
    visitor.visit_name(name);
}

pub fn traverse_gt_tuple(tuple: &GTTuple, visitor: &mut dyn GTVisitor) {
    visitor.visit_tuple(tuple);
    for descriptor in &tuple.descriptors {
        traverse_gt_descriptor(descriptor, visitor);
    }
}

pub fn traverse_gt_reference(reference: &GTReference, visitor: &mut dyn GTVisitor) {
    visitor.visit_reference(reference);
}
