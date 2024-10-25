use genotype_parser::tree::*;

#[cfg(test)]
pub mod mock;

pub trait GTVisitor {
    fn visit_alias(&mut self, _alias: &mut GTAlias) {}

    fn visit_array(&mut self, _array: &mut GTArray) {}

    fn visit_attribute(&mut self, _attribute: &mut GTAttribute) {}

    fn visit_attribute_assignment(&mut self, _assignment: &mut GTAttributeAssignment) {}

    fn visit_attribute_descriptor(&mut self, _descriptor: &mut GTAttributeDescriptor) {}

    fn visit_attribute_key(&mut self, _key: &mut GTAttributeKey) {}

    fn visit_attribute_name(&mut self, _name: &mut GTAttributeName) {}

    fn visit_attribute_property(&mut self, _property: &mut GTAttributeProperty) {}

    fn visit_attribute_value(&mut self, _value: &mut GTAttributeValue) {}

    fn visit_descriptor(&mut self, _descriptor: &mut GTDescriptor) {}

    fn visit_doc(&mut self, _doc: &mut GTDoc) {}

    fn visit_extension(&mut self, _extension: &mut GTExtension) {}

    fn visit_identifier(&mut self, _identifier: &mut GTIdentifier) {}

    fn visit_import(&mut self, _import: &mut GTImport) {}

    fn visit_import_name(&mut self, _name: &mut GTImportName) {}

    fn visit_import_reference(&mut self, _reference: &mut GTImportReference) {}

    fn visit_inline_import(&mut self, _import: &mut GTInlineImport) {}

    fn visit_key(&mut self, _key: &mut GTKey) {}

    fn visit_literal(&mut self, _literal: &mut GTLiteral) {}

    fn visit_module(&mut self, _module: &mut GTModule) {}

    fn visit_object(&mut self, _object: &mut GTObject) {}

    fn visit_object_name(&mut self, _name: &mut GTObjectName) {}

    fn visit_path(&mut self, _path: &mut GTPath) {}

    fn visit_primitive(&mut self, _primitive: &mut GTPrimitive) {}

    fn visit_property(&mut self, _property: &mut GTProperty) {}

    fn visit_reference(&mut self, _reference: &mut GTReference) {}

    fn visit_record(&mut self, _record: &mut GTRecord) {}

    fn visit_record_key(&mut self, _key: &mut GTRecordKey) {}

    fn visit_tuple(&mut self, _tuple: &mut GTTuple) {}

    fn visit_union(&mut self, _union: &mut GTUnion) {}

    fn visit_any(&mut self, _any: &mut GTAny) {}
}
