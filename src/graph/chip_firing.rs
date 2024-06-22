use crate::graph::node::Node;

#[derive(Debug)]
pub struct ChipFiringNode {
    pub id: usize,
    pub name: String,
    pub num_chips: u32,
    pub info: String,
    pub neighbors: Vec<usize>,
}

impl Node for ChipFiringNode {
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

impl ChipFiringNode {
    pub fn new(id: usize, name: &str, num_chips: u32, info: &str, neighbors: Vec<usize>) -> Self {
        Self {
            id,
            name: name.to_string(),
            num_chips,
            info: info.to_string(),
            neighbors,
        }
    }

    pub fn add_neighbor(&mut self, neighbor_id: usize) {
        self.neighbors.push(neighbor_id);
    }
}
