use std::{any::Any, collections::HashMap};


#[derive(Debug)]
pub struct ActionResponse {
    pub outputs: HashMap<String, Box<dyn Any + Send + Sync>>,
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
        V: Any + Send + Sync,
    {
        self.outputs.insert(key.to_string(), Box::new(value));
        self
    }

    pub fn get_string(&self, key: &str) -> Option<String> {
        self.outputs.get(key)?.downcast_ref::<String>().cloned()
    }

    pub fn get_f64(&self, key: &str) -> Option<f64> {
        self.outputs.get(key)?.downcast_ref::<f64>().copied()
    }

    pub fn get_bool(&self, key: &str) -> Option<bool> {
        self.outputs.get(key)?.downcast_ref::<bool>().copied()
    }

    pub fn get_u64(&self, key: &str) -> Option<u64> {
        self.outputs.get(key)?.downcast_ref::<u64>().copied()
    }
}

