use std::{any::Any, collections::HashMap};

use super::variable::Variable;


#[derive(Debug)]
pub struct ActionResponse {
    pub outputs: HashMap<String, Variable>,
}

impl ActionResponse {
    pub fn new() -> Self {
        ActionResponse {
            outputs: HashMap::new(),
        }
    }

    pub fn set_output_field<K, V>(mut self, key: K, value: V) -> Self
    where
        K: ToString,
        V: Into<Variable>,
    {
        self.outputs.insert(key.to_string(), Into::<Variable>::into(value.into()));
        self
    }
}

