use genotype_lang_rs_tree::*;

#[cfg(test)]
pub mod mock;

pub trait RSVisitor {
    fn visit_alias(&mut self, _alias: &mut RSAlias) {}

    fn visit_any(&mut self, _any: &mut RSAny) {}

    fn visit_attribute(&mut self, _attribute: &mut RSAttribute) {}

    fn visit_definition(&mut self, _definition: &mut RSDefinition) {}

    fn visit_dependency(&mut self, _dependency: &mut RSDependency) {}

    fn visit_descriptor(&mut self, _descriptor: &mut RSDescriptor) {}

    fn visit_doc(&mut self, _doc: &mut RSDoc) {}

    fn visit_enum(&mut self, _enum: &mut RSEnum) {}

    fn visit_enum_variant(&mut self, _variant: &mut RSEnumVariant) {}

    fn visit_enum_variant_descriptor(&mut self, _descriptor: &mut RSEnumVariantDescriptor) {}

    fn visit_field(&mut self, _field: &mut RSField) {}

    fn visit_field_name(&mut self, _field_name: &mut RSFieldName) {}

    fn visit_hash_map(&mut self, _hash_map: &mut RSHashMap) {}

    fn visit_identifier(&mut self, _identifier: &mut RSIdentifier) {}

    fn visit_inline_use(&mut self, _inline_use: &mut RSInlineUse) {}

    fn visit_module(&mut self, _module: &mut RSModule) {}

    fn visit_option(&mut self, _option: &mut RSOption) {}

    fn visit_path(&mut self, _path: &mut RSPath) {}

    fn visit_primitive(&mut self, _primitive: &mut RSPrimitive) {}

    fn visit_reference(&mut self, _reference: &mut RSReference) {}

    fn visit_struct(&mut self, _struct: &mut RSStruct) {}

    fn visit_struct_fields(&mut self, _fields: &mut RSStructFields) {}

    fn visit_tuple(&mut self, _tuple: &mut RSTuple) {}

    fn visit_use(&mut self, _use: &mut RSUse) {}

    fn visit_use_name(&mut self, _use_name: &mut RSUseName) {}

    fn visit_use_reference(&mut self, _reference: &mut RSUseReference) {}

    fn visit_vec(&mut self, _vec: &mut RSVec) {}
}
