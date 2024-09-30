use genotype_parser::tree::{
    alias::GTAlias, array::GTArray, descriptor::GTDescriptor, import::GTImport,
    inline_import::GTInlineImport, module::GTModule, name::GTName, object::GTObject,
    primitive::GTPrimitive, property::GTProperty, tuple::GTTuple,
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

    fn visit_inline_import(&mut self, _import: &GTInlineImport) {}

    fn visit_name(&mut self, _name: &GTName) {}
}
