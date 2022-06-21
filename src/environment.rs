use crate::apl_type::AplType;
use std::boxed::Box;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    values: HashMap<String, AplType>,
    pub enclosing: Option<Box<Environment>>,
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn with_enclosing(r: Box<Environment>) -> Environment {
        Environment {
            values: HashMap::new(),
            enclosing: Some(r),
        }
    }

    pub fn extend(&mut self, other: Environment) {
        self.values.extend(other.values);
    }

    pub fn define(&mut self, name: &str, value: AplType) {
        self.values.insert(name.to_string(), value);
    }

    pub fn assign(&mut self, name: &str, value: AplType) {
        if self.values.contains_key(&name.to_string()) {
            self.define(name, value);
        } else if let Some(ref mut enclosing) = self.enclosing {
            enclosing.assign(name, value);
        }
    }

    pub fn get(&self, name: &str) -> Option<AplType> {
        let res = self.values.get(&name.to_string());
        if res.is_some() {
            res.cloned()
        } else if let Some(ref enclosing) = self.enclosing {
            enclosing.get(name)
        } else {
            None
        }
    }

    pub fn contains(&self, name: &str) -> bool {
        match self.values.contains_key(&name.to_string()) {
            true => true,
            false => {
                if let Some(ref enclosing) = self.enclosing {
                    enclosing.contains(name)
                } else {
                    false
                }
            }
        }
    }
}
