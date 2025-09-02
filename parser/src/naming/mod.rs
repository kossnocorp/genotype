use heck::ToPascalCase;
use std::collections::HashSet;

#[derive(Default, Debug, PartialEq, Clone)]
pub struct GTNamingContext {
    claimed: HashSet<String>,
    stack: Vec<GTNamingContextName>,
}

impl GTNamingContext {
    pub fn new() -> Self {
        Default::default()
    }

    /// Pushes a name to the stack.
    pub fn push_name(&mut self, name: GTNamingContextName) {
        self.stack.push(name);
    }

    /// Pops a name from the stack.
    pub fn pop_name(&mut self) {
        self.stack.pop();
    }

    /// Ensures the name is unique by appending an enumerated number if necessary and claims it
    /// so it can't be used again. If name is `None`, it will be generated from the stack using
    /// the entity name (i.e. Object or Array).
    pub fn claim_name(&mut self, name: Option<String>, entity_name: &str) -> String {
        let name = match name {
            Some(name) => name,
            None => self.stack_name(entity_name),
        };

        let name = if self.is_name_claimed(&name) {
            self.enumerate_name(&name)
        } else {
            name
        };

        self.claimed.insert(name.clone());

        name
    }

    /// Claims the name from the stack. The name is built by joining the names in the stack.
    fn stack_name(&self, _entity_name: &str) -> String {
        // Collect the names in the stack until the first named one or the end of the stack.
        let mut stack = vec![];
        for name in self.stack.iter().rev() {
            stack.insert(0, name.as_identifier_string());
            if let GTNamingContextName::Identifier(_) = name {
                break;
            }
        }

        // Join the names in the stack.
        let name = stack
            .iter()
            .map(|segment| segment.to_pascal_case())
            .collect::<Vec<_>>()
            .join("");

        if name.is_empty() {
            // Apply default "Root" name.
            "Root".into()
        } else {
            name
            // // Or combine stack with the entity name.
            // format!("{name}{entity_name}")
        }
    }

    /// Enumerates the name if it's already claimed.
    fn enumerate_name(&self, name: &String) -> String {
        let mut index = 2;
        loop {
            let enumerated_name = format!("{name}{index}");
            if !self.claimed.contains(&enumerated_name) {
                return enumerated_name;
            }
            index += 1;
        }
    }

    /// Checks whether the name is already claimed.
    fn is_name_claimed<Str: AsRef<str>>(&self, name: Str) -> bool {
        self.claimed.contains(name.as_ref())
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum GTNamingContextName {
    /// Identifier name.
    Identifier(String),
    /// Transitive name that does not constitute an identifier, i.e., property name.
    Transitive(String),
}

impl GTNamingContextName {
    pub fn as_string(&self) -> String {
        match self {
            GTNamingContextName::Identifier(name) => name,
            GTNamingContextName::Transitive(name) => name,
        }
        .clone()
    }

    pub fn as_identifier_string(&self) -> String {
        self.as_string().to_pascal_case()
    }
}

// [TODO] Add tests
