use genotype_parser::{GTNamingContext, GTNamingContextName};
use heck::ToPascalCase;

#[derive(Default)]
pub struct GtjTreeConvertContext {
    naming: GTNamingContext,
}

impl GtjTreeConvertContext {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn claim_name(&mut self, name: Option<String>, entity_name: &str) -> String {
        self.naming.claim_name(name, entity_name)
    }

    pub fn enter_name_context<
        BodyFn: FnOnce(&mut GtjTreeConvertContext) -> BodyFnResult,
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
