use crate::category::topology::GrothendieckTopology;
use crate::sheaf::presheaf::Presheaf;

pub struct Sheaf {
    pub presheaf: Presheaf,
    pub topology: GrothendieckTopology,
}

impl Sheaf {
    pub fn new(topology: GrothendieckTopology) -> Self {
        Sheaf {
            presheaf: Presheaf::new(),
            topology,
        }
    }

    pub fn assign(&mut self, object: &str, value: Vec<String>) {
        self.presheaf.assign(object, value);
    }

    pub fn get(&self, object: &str) -> Option<&Vec<String>> {
        self.presheaf.get(object)
    }

    pub fn check_sheaf_condition(&self) -> bool {
        for covering in &self.topology.coverings {
            let mut local_data = Vec::new();
            for morphism in &covering.covers {
                if let Some(data) = self.presheaf.get(&morphism.source) {
                    local_data.push(data.clone());
                } else {
                    return false;
                }
            }
            if !self.sheaf_condition(&local_data) {
                return false;
            }
        }
        true
    }

    fn sheaf_condition(&self, data: &Vec<Vec<String>>) -> bool {
        if data.is_empty() {
            return true;
        }
        let first = &data[0];
        data.iter().all(|d| d == first)
    }
}
