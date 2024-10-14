use pest::iterators::Pair;

use crate::{
    diagnostic::error::GTNodeParseError,
    parser::Rule,
    tree::{GTReference, GTResolve},
};

use super::GTExtension;

impl GTExtension {
    pub fn parse(pair: Pair<'_, Rule>, resolve: &mut GTResolve) -> Result<Self, GTNodeParseError> {
        Ok(GTExtension {
            reference: GTReference::parse(pair.into_inner().next().unwrap(), resolve),
        })
    }
}
