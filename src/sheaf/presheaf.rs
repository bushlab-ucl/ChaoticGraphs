use std::collections::HashMap;

#[derive(Debug)]
pub struct Presheaf {
    pub values: HashMap<String, Vec<String>>,
}

impl Presheaf {
    pub fn new() -> Self {
        Presheaf {
            values: HashMap::new(),
        }
    }

    pub fn assign(&mut self, object: &str, value: Vec<String>) {
        self.values.insert(object.to_string(), value);
    }

    pub fn get(&self, object: &str) -> Option<&Vec<String>> {
        self.values.get(object)
    }
}
