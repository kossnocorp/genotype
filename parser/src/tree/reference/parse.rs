use pest::iterators::Pair;

use crate::{parser::Rule, tree::GTResolve};

use super::GTReference;

impl GTReference {
    pub fn parse(pair: Pair<'_, Rule>, resolve: &mut GTResolve) -> Self {
        let identifier = pair.as_str().into();
        GTReference::Unresolved(identifier)
    }
}
