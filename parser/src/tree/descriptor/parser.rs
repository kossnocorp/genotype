use pest::iterators::Pair;

use crate::parser::Rule;

use super::GTDescriptor;

impl TryFrom<Pair<'_, Rule>> for GTDescriptor {
    type Error = Box<dyn std::error::Error>;

    fn try_from(pair: Pair<'_, Rule>) -> Result<Self, Self::Error> {
        let nullable = pair.as_rule() == Rule::nullable_descriptor;
        let pair = pair.into_inner().next().unwrap(); // [TODO]

        let descriptor = match pair.as_rule() {
            Rule::primitive => GTDescriptor::Primitive(pair.try_into()?),

            Rule::name => GTDescriptor::Name(pair.try_into()?),

            Rule::object => GTDescriptor::Object(pair.try_into()?),

            Rule::array => GTDescriptor::Array(Box::new(pair.try_into()?)),

            Rule::tuple => GTDescriptor::Tuple(pair.try_into()?),

            Rule::descriptor => pair.try_into()?,

            Rule::alias => GTDescriptor::Alias(Box::new(pair.try_into()?)),

            Rule::inline_reference => GTDescriptor::Reference(pair.try_into()?),

            _ => {
                println!("3 ====== unknown rule: {:?}", pair);
                unreachable!("unknown rule");
            }
        };

        if nullable {
            Ok(GTDescriptor::Nullable(Box::new(descriptor)))
        } else {
            Ok(descriptor)
        }
    }
}
