use pest::iterators::Pair;

use crate::{
    parser::Rule,
    tree::{GTReference, GTResolve},
};

use super::GTExtension;

impl GTExtension {
    pub fn parse(
        pair: Pair<'_, Rule>,
        resolve: &mut GTResolve,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(GTExtension {
            reference: GTReference::parse(pair.into_inner().next().unwrap(), resolve),
        })
    }
}
