use genotype_parser::tree::*;

#[cfg(test)]
pub mod mock;

pub trait GTVisitor {
    fn visit_alias(&mut self, _alias: &GTAlias) {}

    fn visit_array(&mut self, _array: &GTArray) {}

    fn visit_descriptor(&mut self, _descriptor: &GTDescriptor) {}

    fn visit_doc(&mut self, _tuple: &GTDoc) {}

    fn visit_identifier(&mut self, _module: &GTIdentifier) {}

    fn visit_import(&mut self, _import: &GTImport) {}

    fn visit_import_name(&mut self, _module: &GTImportName) {}

    fn visit_import_reference(&mut self, _module: &GTImportReference) {}

    fn visit_inline_import(&mut self, _import: &GTInlineImport) {}

    fn visit_key(&mut self, _module: &GTKey) {}

    fn visit_module(&mut self, _module: &GTModule) {}

    fn visit_object(&mut self, _object: &GTObject) {}

    fn visit_path(&mut self, _tuple: &GTPath) {}

    fn visit_primitive(&mut self, _primitive: &GTPrimitive) {}

    fn visit_property(&mut self, _property: &GTProperty) {}

    fn visit_reference(&mut self, _tuple: &GTReference) {}

    fn visit_tuple(&mut self, _tuple: &GTTuple) {}

    fn visit_union(&mut self, _name: &GTUnion) {}
}
