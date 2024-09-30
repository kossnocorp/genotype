use crate::visitor::GTVisitor;

mod alias;
mod array;
mod descriptor;
mod import;
mod module;
mod name;
mod object;
mod primitive;
mod property;
mod reference;
mod tuple;

pub trait GTTraverse {
    fn traverse(&self, visitor: &mut dyn GTVisitor);
}
