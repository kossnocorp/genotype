use genotype_visitor_macro::Visitor;

#[derive(Visitor)]
struct GtNode {
    #[visit]
    value: i32,
}

mod visitor {
    use crate::GtNode;

    pub use genotype_visitor_macro::visitor;

    pub trait Traverse<V: ?Sized> {
        fn traverse(&mut self, visitor: &mut V);
    }

    #[visitor(GtNode)]
    pub struct GtVisitor;
}

fn main() {}
