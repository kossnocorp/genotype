use crate::visitor::GTVisitor;

mod alias;
mod any;
mod array;
mod attribute;
mod attribute_assignment;
mod attribute_descriptor;
mod attribute_key;
mod attribute_name;
mod attribute_property;
mod attribute_value;
mod descriptor;
mod doc;
mod extension;
mod identifier;
mod import;
mod import_name;
mod import_reference;
mod inline_import;
mod key;
mod literal;
mod module;
mod object;
mod object_name;
mod path;
mod primitive;
mod property;
mod record;
mod record_key;
mod reference;
mod tuple;
mod union;

pub trait GTTraverse {
    fn traverse(&mut self, visitor: &mut dyn GTVisitor);
}
