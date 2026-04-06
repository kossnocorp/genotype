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
        fn traverse(&self, visitor: &mut V);
    }

    pub trait TraverseMut<V: ?Sized> {
        fn traverse_mut(&mut self, visitor: &mut V);
    }

    impl<V: ?Sized, T> Traverse<V> for Option<T>
    where
        T: Traverse<V>,
    {
        fn traverse(&self, visitor: &mut V) {
            if let Some(inner) = self {
                inner.traverse(visitor);
            }
        }
    }

    impl<V: ?Sized, T> TraverseMut<V> for Option<T>
    where
        T: TraverseMut<V>,
    {
        fn traverse_mut(&mut self, visitor: &mut V) {
            if let Some(inner) = self {
                inner.traverse_mut(visitor);
            }
        }
    }

    impl<V: ?Sized, T> Traverse<V> for Vec<T>
    where
        T: Traverse<V>,
    {
        fn traverse(&self, visitor: &mut V) {
            for inner in self {
                inner.traverse(visitor);
            }
        }
    }

    impl<V: ?Sized, T> TraverseMut<V> for Vec<T>
    where
        T: TraverseMut<V>,
    {
        fn traverse_mut(&mut self, visitor: &mut V) {
            for inner in self {
                inner.traverse_mut(visitor);
            }
        }
    }

    impl<V: ?Sized, T> Traverse<V> for Box<T>
    where
        T: Traverse<V>,
    {
        fn traverse(&self, visitor: &mut V) {
            self.as_ref().traverse(visitor);
        }
    }

    impl<V: ?Sized, T> TraverseMut<V> for Box<T>
    where
        T: TraverseMut<V>,
    {
        fn traverse_mut(&mut self, visitor: &mut V) {
            self.as_mut().traverse_mut(visitor);
        }
    }

    #[visitor(nodes(GtLeaf, GtNode), mut_trait = GtVisitorMut)]
    pub struct GtVisitor;
}

struct Walker;

impl visitor::GtVisitor for Walker {}
impl visitor::GtVisitorMut for Walker {}

fn main() {
    let mut node = GtNode {
        child: GtLeaf,
        maybe: Some(GtLeaf),
        many: vec![GtLeaf],
        boxed: Box::new(GtLeaf),
        ignored: 1,
    };

    let mut walker = Walker;
    visitor::Traverse::traverse(&node, &mut walker);
    visitor::TraverseMut::traverse_mut(&mut node, &mut walker);
    assert_eq!(node.ignored, 1);
}
