use genotype_parser::tree::{
    alias::GTAlias, array::GTArray, descriptor::GTDescriptor, import::GTImport, module::GTModule,
    name::GTName, object::GTObject, primitive::GTPrimitive, property::GTProperty,
    reference::GTReference, tuple::GTTuple,
};

pub trait GTVisitor {
    fn visit_module(&mut self, _module: &GTModule) {}

    fn visit_import(&mut self, _import: &GTImport) {}

    fn visit_alias(&mut self, _alias: &GTAlias) {}

    fn visit_tuple(&mut self, _tuple: &GTTuple) {}

    fn visit_array(&mut self, _array: &GTArray) {}

    fn visit_descriptor(&mut self, _descriptor: &GTDescriptor) {}

    fn visit_object(&mut self, _object: &GTObject) {}

    fn visit_primitive(&mut self, _primitive: &GTPrimitive) {}

    fn visit_property(&mut self, _property: &GTProperty) {}

    fn visit_reference(&mut self, _reference: &GTReference) {}

    fn visit_name(&mut self, _name: &GTName) {}
}
