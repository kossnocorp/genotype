use genotype_visitor_macro::Visitor;

#[derive(Visitor)]
struct GtLeaf;

#[derive(Visitor)]
struct GtNode {
    #[visit]
    child: GtLeaf,
    #[visit]
    maybe: Option<GtLeaf>,
    #[visit]
    many: Vec<GtLeaf>,
    #[visit]
    boxed: Box<GtLeaf>,
    ignored: i32,
}

mod visitor {
    use crate::{GtLeaf, GtNode};

    pub use genotype_visitor_macro::visitor;

    pub trait Traverse<V: ?Sized> {
        fn traverse(&mut self, visitor: &mut V);
    }

    impl<V: ?Sized, T> Traverse<V> for Option<T>
    where
        T: Traverse<V>,
    {
        fn traverse(&mut self, visitor: &mut V) {
            if let Some(inner) = self {
                inner.traverse(visitor);
            }
        }
    }

    impl<V: ?Sized, T> Traverse<V> for Vec<T>
    where
        T: Traverse<V>,
    {
        fn traverse(&mut self, visitor: &mut V) {
            for inner in self {
                inner.traverse(visitor);
            }
        }
    }

    impl<V: ?Sized, T> Traverse<V> for Box<T>
    where
        T: Traverse<V>,
    {
        fn traverse(&mut self, visitor: &mut V) {
            self.as_mut().traverse(visitor);
        }
    }

    #[visitor(GtLeaf, GtNode)]
    pub struct GtVisitor;
}

struct Walker;

impl visitor::GtVisitor for Walker {}

fn main() {
    let mut node = GtNode {
        child: GtLeaf,
        maybe: Some(GtLeaf),
        many: vec![GtLeaf],
        boxed: Box::new(GtLeaf),
        ignored: 1,
    };

    let mut walker = Walker;
    visitor::Traverse::traverse(&mut node, &mut walker);
    assert_eq!(node.ignored, 1);
}
