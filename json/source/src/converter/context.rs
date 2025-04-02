use genotype_parser::{GTNamingContext, GTNamingContextName};
use heck::ToPascalCase;

#[derive(Default)]
pub struct GtjConvertContext {
    naming: GTNamingContext,
}

impl GtjConvertContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn claim_name(&mut self, name: Option<String>, entity_name: &str) -> String {
        self.naming.claim_name(name, entity_name)
    }

    pub fn enter_name_context<
        BodyFn: FnOnce(&mut GtjConvertContext) -> BodyFnResult,
        BodyFnResult,
    >(
        &mut self,
        name: GTNamingContextName,
        body: BodyFn,
    ) -> BodyFnResult {
        self.naming.push_name(name);
        let result = body(self);
        self.naming.pop_name();
        result
    }
}

pub enum GtjConvertContextParent {
    ArrayItem,
    ObjectProperty(String),
}

impl GtjConvertContextParent {
    pub fn name(&self) -> String {
        match self {
            GtjConvertContextParent::ArrayItem => "Item".to_string(),
            GtjConvertContextParent::ObjectProperty(name) => name.to_pascal_case(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    // #[test]
    // fn test_claim_name_root() {
    //     let mut context = GtjConvertContext::default();
    //     let result = context.claim_name(None);
    //     assert_eq!(result, (GTNamingContextNameToken::Derived, "Root".into()));
    // }

    // #[test]
    // fn test_claim_name_property() {
    //     let mut context = GtjConvertContext::default();
    //     context.enter_name_context((GTNamingContextNameToken::Named, "Hello"), |context| {
    //         let result = context.claim_name(None);
    //         assert_eq!(result, (GTNamingContextNameToken::Named, "Hello"));
    //     });
    //     let result = context.claim_name(None);
    //     assert_eq!(result, (GTNamingContextNameToken::Derived, "Root".into()));
    // }

    // #[test]
    // fn test_claim_name_nested() {
    //     let mut context = GtjConvertContext::default();
    //     context.enter_name_context("Hello", |context| {
    //         context.enter_name_context("Item", |context| {
    //             context.enter_name_context("World", |context| {
    //                 let result = context.claim_name(None);
    //                 assert_eq!(result, "HelloItemWorld");
    //             });
    //         });
    //     });
    //     let result = context.claim_name(None);
    //     assert_eq!(result, (GTNamingContextNameToken::Derived, "Root".into()));
    // }

    // #[test]
    // fn test_claim_name_unique() {
    //     let mut context = GtjConvertContext::default();
    //     context.enter_name_context("Item", |context| {
    //         let result = context.claim_name(None);
    //         assert_eq!(result, "Item");
    //     });
    //     context.enter_name_context("Item", |context| {
    //         let result = context.claim_name(None);
    //         assert_eq!(result, "Item2");
    //     });
    //     let result = context.claim_name(None);
    //     assert_eq!(result, (GTNamingContextNameToken::Derived, "Root".into()));
    // }
}
