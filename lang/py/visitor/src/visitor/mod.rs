use genotype_lang_py_tree::*;

#[cfg(test)]
pub mod mock;

pub trait PYVisitor {
    fn visit_alias(&mut self, _alias: &mut PYAlias) {}

    fn visit_any(&mut self, _any: &mut PYAny) {}

    fn visit_class(&mut self, _object: &mut PYClass) {}

    fn visit_definition(&mut self, _definition: &mut PYDefinition) {}

    fn visit_dependency(&mut self, _dependency: &mut PYDependency) {}

    fn visit_descriptor(&mut self, _descriptor: &mut PYDescriptor) {}

    fn visit_dict(&mut self, _dict: &mut PYDict) {}

    fn visit_dict_key(&mut self, _key: &mut PYDictKey) {}

    fn visit_doc(&mut self, _doc: &mut PYDoc) {}

    fn visit_extension(&mut self, _extension: &mut PYExtension) {}

    fn visit_identifier(&mut self, _identifier: &mut PYIdentifier) {}

    fn visit_import(&mut self, _import: &mut PYImport) {}

    fn visit_import_name(&mut self, _name: &mut PYImportName) {}

    fn visit_import_reference(&mut self, _reference: &mut PYImportReference) {}

    fn visit_key(&mut self, _key: &mut PYKey) {}

    fn visit_list(&mut self, _array: &mut PYList) {}

    fn visit_literal(&mut self, _literal: &mut PYLiteral) {}

    fn visit_module(&mut self, _module: &mut PYModule) {}

    fn visit_path(&mut self, _path: &mut PYPath) {}

    fn visit_primitive(&mut self, _primitive: &mut PYPrimitive) {}

    fn visit_property(&mut self, _property: &mut PYProperty) {}

    fn visit_reference(&mut self, _reference: &mut PYReference) {}

    fn visit_tuple(&mut self, _tuple: &mut PYTuple) {}

    fn visit_union(&mut self, _union: &mut PYUnion) {}

    fn visit_newtype(&mut self, _newtype: &mut PYNewtype) {}
}
