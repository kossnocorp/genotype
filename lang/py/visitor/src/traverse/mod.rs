use crate::visitor::PYVisitor;

mod alias;
mod any;
mod class;
mod definition;
mod dependency;
mod descriptor;
mod dict;
mod dict_key;
mod doc;
mod extension;
mod identifier;
mod import;
mod import_name;
mod import_reference;
mod key;
mod list;
mod literal;
mod module;
mod newtype;
mod path;
mod primitive;
mod property;
mod reference;
mod tuple;
mod union;

pub trait PYTraverse {
    fn traverse(&mut self, visitor: &mut dyn PYVisitor);
}
