use crate::category::category::Category;
use crate::category::topology::GrothendieckTopology;
use crate::graph::node::Node;
use crate::sheaf::sheaf::Sheaf;

pub struct Graph<T: Node> {
    pub nodes: Vec<T>,
    pub category: Category,
    pub topology: GrothendieckTopology,
    pub sheaf: Sheaf,
}

impl<T: Node> Graph<T> {
    pub fn new(topology: GrothendieckTopology) -> Self {
        let category = Category::new();
        let sheaf = Sheaf::new(topology.clone());
        Self {
            nodes: Vec::new(),
            category,
            topology,
            sheaf,
        }
    }

    pub fn add_node(&mut self, node: T) {
        self.category.add_object(node.get_name());
        self.sheaf.assign(node.get_name(), vec![node.get_info()]);
        self.nodes.push(node);
    }

    pub fn get_node(&self, id: usize) -> Option<&T> {
        self.nodes.iter().find(|n| n.get_id() == id)
    }

    pub fn update_node(&mut self, id: usize, update_fn: impl Fn(&mut T)) {
        if let Some(node) = self.nodes.iter_mut().find(|n| n.get_id() == id) {
            update_fn(node);
            self.sheaf.assign(node.get_name(), vec![node.get_info()]);
        }
    }

    pub fn add_edge(&mut self, node_id: usize, neighbor_id: usize, is_isomorphism: bool) {
        if let (Some(node), Some(neighbor)) = (self.get_node(node_id), self.get_node(neighbor_id)) {
            let node_name = node.get_name().to_string();
            let neighbor_name = neighbor.get_name().to_string();
            self.category
                .add_morphism(&node_name, &neighbor_name, is_isomorphism);
        }
    }

    pub fn laplacian_matrix(&self) -> Vec<Vec<i32>> {
        let n = self.nodes.len();
        let mut matrix = vec![vec![0; n]; n];

        for node in &self.nodes {
            let i = node.get_id();
            let degree = node.get_neighbors().len() as i32;
            matrix[i][i] = degree;

            for &neighbor in &node.get_neighbors() {
                let j = neighbor;
                matrix[i][j] = -1;
                matrix[j][i] = -1; // Assuming undirected graph
            }
        }

        matrix
    }

    pub fn check_sheaf_condition(&self) -> bool {
        if self.sheaf.check_sheaf_condition() {
            // Additional topology checks
            for covering in &self.topology.coverings {
                for morphism in &covering.covers {
                    if !self.topology.is_covering(morphism) {
                        return false;
                    }
                }
            }
            true
        } else {
            false
        }
    }
}
