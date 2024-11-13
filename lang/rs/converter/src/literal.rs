use std::path::Display;

use genotype_lang_rs_tree::{RSAttribute, RSContextResolve, RSStruct};
use genotype_parser::tree::GTLiteral;

use crate::{context::RSConvertContext, convert::RSConvert};

impl RSConvert<RSStruct> for GTLiteral {
    fn convert(&self, context: &mut RSConvertContext) -> RSStruct {
        let value = match self {
            GTLiteral::Boolean(_, value) => value.to_string(),
            GTLiteral::Integer(_, value) => value.to_string(),
            GTLiteral::Float(_, value) => value.to_string(),
            GTLiteral::String(_, value) => format!(r#""{value}""#),
        };

        // [TODO] Resolve and import the runtime

        RSStruct {
            // [TODO]
            doc: None,
            attributes: vec![RSAttribute(format!("literal({value})"))],
            extensions: vec![],
            // [TODO] Generate name
            name: "Literal".into(),
            properties: vec![],
        }
        // .resolve(context)
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_rs_tree::*;
    use pretty_assertions::assert_eq;

    use crate::context::RSConvertContext;

    use super::*;

    // #[test]
    // fn test_convert() {
    //     assert_eq!(
    //         RSLiteral::Boolean(true),
    //         GTLiteral::Boolean((0, 0).into(), true).convert(&mut RSConvertContext::default()),
    //     );
    //     assert_eq!(
    //         RSLiteral::Integer(-123),
    //         GTLiteral::Integer((0, 0).into(), -123).convert(&mut RSConvertContext::default()),
    //     );
    //     assert_eq!(
    //         RSLiteral::Float(1.23),
    //         GTLiteral::Float((0, 0).into(), 1.23).convert(&mut RSConvertContext::default()),
    //     );
    //     assert_eq!(
    //         RSLiteral::String("Hello, world!".into()),
    //         GTLiteral::String((0, 0).into(), "Hello, world!".into())
    //             .convert(&mut RSConvertContext::default()),
    //     );
    // }

    // #[test]
    // fn test_convert_resolve() {
    //     let mut context = Default::default();
    //     assert_eq!(
    //         GTLiteral::Boolean((0, 0).into(), false).convert(&mut context),
    //         RSLiteral::Boolean(false)
    //     );
    //     assert_eq!(
    //         context.as_dependencies(),
    //         vec![(RSDependency::Typing, "Literal".into())]
    //     );
    // }
}
