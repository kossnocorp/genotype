use genotype_visitor_macro::Visitor;

#[derive(Visitor)]
struct GTLeaf;

#[derive(Visitor)]
struct GTNode {
    #[visit]
    child: GTLeaf,
    #[visit]
    maybe: Option<GTLeaf>,
    #[visit]
    many: Vec<GTLeaf>,
    #[visit]
    boxed: Box<GTLeaf>,
    ignored: i32,
}

mod visitor {
    use crate::{GTLeaf, GTNode};

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

    #[visitor(GTLeaf, GTNode)]
    pub struct GTVisitor;
}

struct Walker;

impl visitor::GTVisitor for Walker {}

fn main() {
    let mut node = GTNode {
        child: GTLeaf,
        maybe: Some(GTLeaf),
        many: vec![GTLeaf],
        boxed: Box::new(GTLeaf),
        ignored: 1,
    };

    let mut walker = Walker;
    visitor::Traverse::traverse(&mut node, &mut walker);
    assert_eq!(node.ignored, 1);
}
