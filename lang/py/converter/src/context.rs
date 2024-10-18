use std::vec;

use genotype_lang_py_tree::{PYContext, PYDefinition, PYIdentifier, PYOptions, PYPath};

use crate::resolve::PYConvertResolve;

pub struct PYConvertContext {
    pub resolve: PYConvertResolve,
    // [TODO] It should not be a property and struct functions that collects into a vector instead
    hoisting: bool,
    hoist: Box<dyn Fn(PYDefinition)>,
    pub tree: PYContext,
    pub options: PYOptions,
    defined: Vec<PYIdentifier>,
    hoist_defined: Vec<PYIdentifier>,
}

impl PYConvertContext {
    pub fn new(resolve: PYConvertResolve, hoist: Box<dyn Fn(PYDefinition)>) -> Self {
        Self {
            resolve,
            hoisting: false,
            hoist,
            tree: PYContext::new(),
            options: PYOptions::default(),
            defined: vec![],
            hoist_defined: vec![],
        }
    }

    pub fn import(&mut self, path: PYPath, name: PYIdentifier) {
        self.tree.imports.insert((path, name));
    }

    pub fn hoist<HoistFn>(&mut self, mut hoist_fn: HoistFn) -> PYIdentifier
    where
        HoistFn: FnMut(&mut PYConvertContext) -> PYDefinition,
    {
        self.hoisting = true;
        let definition = hoist_fn(self);
        let name = definition.name().clone();
        (self.hoist)(definition);
        self.hoisting = false;
        name
    }

    pub fn define(&mut self, identifier: &PYIdentifier) {
        if self.hoisting {
            self.hoist_defined.push(identifier.clone());
        } else {
            self.defined.push(identifier.clone());
        }
    }

    pub fn is_forward(&self, identifier: &PYIdentifier) -> bool {
        let is_defined = self.defined.contains(identifier)
            || (self.hoisting && self.hoist_defined.contains(identifier));
        !is_defined
    }

    pub fn pop_hoisted(&mut self) {
        self.defined.extend(self.hoist_defined.drain(..));
        // [TODO] Return hoisted definitions here
    }
}

impl Default for PYConvertContext {
    fn default() -> Self {
        Self {
            resolve: PYConvertResolve::new(),
            hoist: Box::new(|_| {}),
            hoisting: false,
            tree: PYContext::new(),
            options: PYOptions::default(),
            defined: vec![],
            hoist_defined: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use genotype_lang_py_tree::*;
    use pretty_assertions::assert_eq;

    use crate::mock::mock_context;

    use super::*;

    #[test]
    fn test_hoist() {
        let (hoisted, context) = mock_context();
        let mut context = context;
        context.hoist(|_| {
            PYDefinition::Alias(PYAlias {
                name: "Name".into(),
                descriptor: PYDescriptor::Primitive(PYPrimitive::Boolean),
            })
        });
        assert_eq!(
            hoisted.lock().unwrap().clone(),
            vec![PYDefinition::Alias(PYAlias {
                name: "Name".into(),
                descriptor: PYDescriptor::Primitive(PYPrimitive::Boolean),
            })],
        );
    }

    #[test]
    fn test_define() {
        let mut context = PYConvertContext::default();
        context.define(&"Name".into());
        assert_eq!(context.defined, vec!["Name".into()]);
    }

    #[test]
    fn test_is_forward() {
        let mut context = PYConvertContext::default();
        context.define(&"Name".into());
        assert_eq!(context.is_forward(&"Name".into()), false);
        assert_eq!(context.is_forward(&"Other".into()), true);
    }
}
