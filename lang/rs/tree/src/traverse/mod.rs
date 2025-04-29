use crate::prelude::internal::*;

mod alias;
mod any;
mod attribute;
mod definition;
mod dependency;
mod descriptor;
mod doc;
mod r#enum;
mod enum_variant;
mod enum_variant_descriptor;
mod field;
mod field_name;
mod identifier;
mod inline_use;
mod map;
mod module;
mod option;
mod path;
mod primitive;
mod reference;
mod r#struct;
mod struct_fields;
mod tuple;
mod r#use;
mod use_name;
mod use_reference;
mod vec;

pub trait RSTraverse {
    fn traverse(&mut self, visitor: &mut dyn RSVisitor);
}
