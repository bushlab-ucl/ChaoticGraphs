use crate::sheaf::sheaf::Sheaf;

pub fn check_sum_condition(sheaf: &Sheaf, threshold: i32) -> bool {
    for (_node, data) in &sheaf.presheaf.values {
        let sum: i32 = data.iter().map(|d| d.parse::<i32>().unwrap_or(0)).sum();
        if sum > threshold {
            return false;
        }
    }
    true
}

pub fn check_hopfield_state_condition(sheaf: &Sheaf) -> bool {
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
