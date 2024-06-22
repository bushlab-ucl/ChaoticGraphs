use crate::graph::node::Node;

#[derive(Debug)]
pub struct HopfieldNode {
    pub id: usize,
    pub name: String,
    pub state: i32,
    pub info: String,
    pub neighbors: Vec<usize>,
    pub weights: Vec<f64>,
}

impl Node for HopfieldNode {
    fn get_id(&self) -> usize {
        self.id
    }

    fn get_neighbors(&self) -> Vec<usize> {
        self.neighbors.clone()
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_info(&self) -> String {
        self.info.clone()
    }

    fn set_info(&mut self, info: String) {
        self.info = info;
    }
}

impl HopfieldNode {
    pub fn new(
        id: usize,
        name: &str,
        state: i32,
        info: &str,
        neighbors: Vec<usize>,
        weights: Vec<f64>,
    ) -> Self {
        Self {
            id,
            name: name.to_string(),
            state,
            info: info.to_string(),
            neighbors,
            weights,
        }
    }

    pub fn add_neighbor(&mut self, neighbor_id: usize) {
        self.neighbors.push(neighbor_id);
    }
}
