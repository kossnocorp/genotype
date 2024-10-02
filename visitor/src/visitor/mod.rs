use genotype_parser::tree::*;

#[cfg(test)]
pub mod mock;

pub trait GTVisitor {
    fn visit_alias(&mut self, _alias: &mut GTAlias) {}

    fn visit_array(&mut self, _array: &mut GTArray) {}

    fn visit_descriptor(&mut self, _descriptor: &mut GTDescriptor) {}

    fn visit_doc(&mut self, _tuple: &mut GTDoc) {}

    fn visit_identifier(&mut self, _module: &mut GTIdentifier) {}

    fn visit_import(&mut self, _import: &mut GTImport) {}

    fn visit_import_name(&mut self, _module: &mut GTImportName) {}

    fn visit_import_reference(&mut self, _module: &mut GTImportReference) {}

    fn visit_inline_import(&mut self, _import: &mut GTInlineImport) {}

    fn visit_key(&mut self, _module: &mut GTKey) {}

    fn visit_module(&mut self, _module: &mut GTModule) {}

    fn visit_object(&mut self, _object: &mut GTObject) {}

    fn visit_path(&mut self, _tuple: &mut GTPath) {}

    fn visit_primitive(&mut self, _primitive: &mut GTPrimitive) {}

    fn visit_property(&mut self, _property: &mut GTProperty) {}

    fn visit_reference(&mut self, _tuple: &mut GTReference) {}

    fn visit_tuple(&mut self, _tuple: &mut GTTuple) {}

    fn visit_union(&mut self, _name: &mut GTUnion) {}
}
