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
        fn traverse(&self, visitor: &mut V);
    }

    pub trait TraverseMut<V: ?Sized> {
        fn traverse_mut(&mut self, visitor: &mut V);
    }

    #[visitor(nodes(GtNode), mut_trait = GtVisitorMut)]
    pub struct GtVisitor;
}

fn main() {}
