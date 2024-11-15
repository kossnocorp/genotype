use genotype_lang_rs_tree::{RSIdentifier, RSKey};
use heck::ToPascalCase;

use super::RSConvertContext;

#[derive(Debug, PartialEq, Clone)]
pub enum RSContextParent {
    Alias(RSIdentifier),
    Definition(RSIdentifier),
    Property(RSKey),
    EnumVariant(RSIdentifier),
    Hoist,
}

impl RSContextParent {
    pub fn name(&self) -> String {
        match self {
            Self::Alias(identifier) => identifier.0.clone(),
            Self::Definition(identifier) => identifier.0.clone(),
            Self::Property(key) => key.0.clone(),
            Self::EnumVariant(identifier) => identifier.0.clone(),
            Self::Hoist => panic!("Cannot get name of hoist"),
        }
    }
}

impl From<RSKey> for RSContextParent {
    fn from(key: RSKey) -> Self {
        RSContextParent::Property(key)
    }
}

impl RSConvertContext {
    pub fn enter_parent(&mut self, parent: RSContextParent) {
        self.parents.push(parent);
    }

    pub fn exit_parent(&mut self) {
        // [TODO]
        self.parents.pop().expect("Expected parent to exist");
    }

    pub fn name_child(&self, name: &str) -> RSIdentifier {
        let mut segments = vec![];
        for parent in self.parents.iter().rev() {
            match parent {
                RSContextParent::Hoist => break,
                _ => segments.push(parent.name().to_pascal_case()),
            }
        }
        segments.reverse();
        segments.push(name.to_pascal_case());
        segments.join("").into()
    }
}

#[cfg(test)]
mod tests {
    use crate::context::{naming::RSContextParent, RSConvertContext};

    #[test]
    fn test_name_child() {
        let mut context = RSConvertContext::default();
        context.enter_parent(RSContextParent::Definition("Person".into()));
        context.enter_parent(RSContextParent::Property("name".into()));

        assert_eq!(context.name_child("value"), "PersonNameValue".into());
    }

    #[test]
    fn test_name_hoisted_child() {
        let mut context = RSConvertContext::default();
        context.enter_parent(RSContextParent::Definition("Person".into()));
        context.enter_parent(RSContextParent::Property("name".into()));
        context.enter_parent(RSContextParent::Hoist);
        context.enter_parent(RSContextParent::Definition("Name".into()));

        assert_eq!(context.name_child("union"), "NameUnion".into());
    }
}
