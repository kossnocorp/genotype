use crate::visitor::GTVisitor;

mod alias;
mod array;
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
mod reference;
mod tuple;
mod union;

pub trait GTTraverse {
    fn traverse(&mut self, visitor: &mut dyn GTVisitor);
}
