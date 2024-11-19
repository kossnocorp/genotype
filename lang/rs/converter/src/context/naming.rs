use genotype_lang_rs_tree::{RSFieldName, RSIdentifier};
use heck::ToPascalCase;

use super::RSConvertContext;

#[derive(Debug, PartialEq, Clone)]
pub enum RSContextParent {
    /// Alias parent. Defines the name that children can claim unless there is an anonymous parent
    /// between them.
    Alias(RSIdentifier),
    /// Anonymous parent that prevents children from taking the alias name, when they for example
    /// are part of a tuple.
    Anonymous,
    Definition(RSIdentifier),
    Field(RSFieldName),
    EnumVariant(RSIdentifier),
    Hoist,
}

impl RSContextParent {
    pub fn name(&self) -> String {
        match self {
            Self::Alias(identifier) => identifier.0.clone(),
            Self::Definition(identifier) => identifier.0.clone(),
            Self::Field(key) => key.0.clone(),
            Self::EnumVariant(identifier) => identifier.0.clone(),
            Self::Anonymous => panic!("Cannot get name of anonymous parent"),
            Self::Hoist => panic!("Cannot get name of hoist parent"),
        }
    }
}

impl From<RSFieldName> for RSContextParent {
    fn from(key: RSFieldName) -> Self {
        RSContextParent::Field(key)
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

    pub fn name_child(&self, name: Option<&str>) -> RSIdentifier {
        let mut segments = vec![];
        println!("|||||||||||| parents {:?}", self.parents);
        for parent in self.parents.iter().rev() {
            match parent {
                // [TODO] Kill hoist and variant altogether?
                RSContextParent::Hoist | RSContextParent::EnumVariant(_) => continue,

                _ => {
                    segments.push(parent.name().to_pascal_case());
                    if let RSContextParent::Definition(_) = parent {
                        break;
                    }
                }
            }
        }
        segments.reverse();
        if let Some(name) = name {
            segments.push(name.to_pascal_case());
        }
        segments.join("").into()
    }

    /// Tries claiming the alias from the parent, i.e. when naming literals:
    ///     HelloWorld = "hello-world"
    pub fn claim_alias(&mut self) -> Option<RSIdentifier> {
        if let Some(RSContextParent::Alias(identifier)) = self.parents.last() {
            return Some(identifier.clone());
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::context::{naming::RSContextParent, RSConvertContext};

    #[test]
    fn test_name_child() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Definition("Person".into()));
        context.enter_parent(RSContextParent::Field("name".into()));

        assert_eq!(context.name_child(Some("value")), "PersonNameValue".into());
    }

    #[test]
    fn test_name_hoisted_child() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Definition("Person".into()));
        context.enter_parent(RSContextParent::Field("name".into()));
        context.enter_parent(RSContextParent::Hoist);
        context.enter_parent(RSContextParent::Definition("Name".into()));

        assert_eq!(context.name_child(Some("union")), "NameUnion".into());
    }

    #[test]
    fn test_claim_alias_deep() {
        let mut context = RSConvertContext::empty("module".into());
        context.enter_parent(RSContextParent::Alias("Person".into()));
        assert_eq!(context.claim_alias(), Some("Person".into()));
        context.enter_parent(RSContextParent::Anonymous);
        assert_eq!(context.claim_alias(), None);
    }
}
