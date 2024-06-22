use crate::sheaf::sheaf::Sheaf;

pub trait SheafCondition {
    fn check(&self, sheaf: &Sheaf) -> bool;
}

pub struct SumCondition {
    pub threshold: i32,
}

// check if the sum of the data in each node is less than the threshold
impl SheafCondition for SumCondition {
    fn check(&self, sheaf: &Sheaf) -> bool {
        for (_node, data) in &sheaf.presheaf.values {
            let sum: i32 = data.iter().map(|d| d.parse::<i32>().unwrap_or(0)).sum();
            if sum > self.threshold {
                return false;
            }
        }
        true
    }
}

pub struct HopfieldStateCondition;

// check if all the states in the sheaf are the same
impl SheafCondition for HopfieldStateCondition {
    fn check(&self, sheaf: &Sheaf) -> bool {
        let mut states = vec![];
        for (_node, data) in &sheaf.presheaf.values {
            for state in data {
                if !states.contains(state) {
                    states.push(state.clone());
                }
            }
        }
        states.len() <= 1
    }
}
