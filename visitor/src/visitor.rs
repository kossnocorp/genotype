use genotype_parser::tree::{
    alias::Alias, array::Array, descriptor::Descriptor, import::Import, module::Module,
    object::Object, primitive::Primitive, property::Property, reference::Reference, tuple::Tuple,
};

pub trait Visitor {
    fn visit_module(&mut self, _module: &Module) {}

    fn visit_import(&mut self, _import: &Import) {}

    fn visit_alias(&mut self, _alias: &Alias) {}

    fn visit_tuple(&mut self, _tuple: &Tuple) {}

    fn visit_array(&mut self, _array: &Array) {}

    fn visit_descriptor(&mut self, _descriptor: &Descriptor) {}

    fn visit_object(&mut self, _object: &Object) {}

    fn visit_primitive(&mut self, _primitive: &Primitive) {}

    fn visit_property(&mut self, _property: &Property) {}

    fn visit_reference(&mut self, _reference: &Reference) {}

    fn visit_name(&mut self, _name: &String) {}
}
