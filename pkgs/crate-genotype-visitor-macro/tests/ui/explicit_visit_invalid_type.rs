use genotype_visitor_macro::Visitor;

#[derive(Visitor)]
struct GTNode {
    #[visit]
    value: i32,
}

mod visitor {
    use crate::GTNode;

    pub use genotype_visitor_macro::visitor;

    pub trait Traverse<V: ?Sized> {
        fn traverse(&mut self, visitor: &mut V);
    }

    #[visitor(GTNode)]
    pub struct GTVisitor;
}

fn main() {}
