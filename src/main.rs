use chaotic_networks::category::topology::GrothendieckTopology;
use chaotic_networks::graph::chip_firing::ChipFiringNode;
use chaotic_networks::graph::graph::Graph;
use chaotic_networks::graph::hopfield::HopfieldNode;
use chaotic_networks::graph::node::Node;
use chaotic_networks::sheaf::conditions::{HopfieldStateCondition, SumCondition};
use chaotic_networks::simulation::chip_firing::simulate_chip_firing;
use chaotic_networks::simulation::hopfield::simulate_hopfield_network;

fn main() {
    let max_iterations = 1000;

    // Define nodes and topology
    let nodes = vec![
        "Node0".to_string(),
        "Node1".to_string(),
        "Node2".to_string(),
    ];
    let topology = GrothendieckTopology::discrete(nodes);

    let sum_condition = SumCondition { threshold: 10 };
    let hopfield_condition = HopfieldStateCondition;

    // Example of creating and simulating a chip firing graph
    let mut chip_firing_graph = Graph::new(topology.clone());
    chip_firing_graph.add_node(ChipFiringNode::new(0, "Node0", 4, "Initial", vec![1, 2]));
    chip_firing_graph.add_node(ChipFiringNode::new(1, "Node1", 0, "Initial", vec![0, 2]));
    chip_firing_graph.add_node(ChipFiringNode::new(2, "Node2", 0, "Initial", vec![0, 1]));

    chip_firing_graph.add_edge(0, 1, true);
    chip_firing_graph.add_edge(0, 2, true);

    println!("----");
    println!("Simulating chip firing graph.");
    println!("----");
    simulate_chip_firing(&mut chip_firing_graph, max_iterations, &sum_condition);
    for node in &chip_firing_graph.nodes {
        println!(
            "Node {} has {} chips and info: {}",
            node.get_id(),
            node.num_chips,
            node.get_info()
        );
    }

    // Compute and print the Laplacian matrix for the chip firing graph
    let laplacian = chip_firing_graph.laplacian_matrix();
    println!("Laplacian matrix:");
    for row in laplacian {
        println!("{:?}", row);
    }

    // Example of creating and simulating a Hopfield network graph
    let mut hopfield_graph = Graph::new(topology.clone());
    hopfield_graph.add_node(HopfieldNode::new(
        0,
        "Node0",
        1,
        "Initial",
        vec![1, 2],
        vec![1.0, -1.0],
    ));
    hopfield_graph.add_node(HopfieldNode::new(
        1,
        "Node1",
        -1,
        "Initial",
        vec![0, 2],
        vec![1.0, 1.0],
    ));
    hopfield_graph.add_node(HopfieldNode::new(
        2,
        "Node2",
        1,
        "Initial",
        vec![0, 1],
        vec![-1.0, 1.0],
    ));

    hopfield_graph.add_edge(0, 1, true);
    hopfield_graph.add_edge(0, 2, true);

    println!("----");
    println!("Simulating Hopfield network.");
    println!("----");
    simulate_hopfield_network(&mut hopfield_graph, max_iterations, &hopfield_condition);
    for node in &hopfield_graph.nodes {
        println!(
            "Node {} has state {} and info: {}",
            node.get_id(),
            node.state,
            node.get_info()
        );
    }

    // Compute and print the Laplacian matrix for the Hopfield network
    let laplacian = hopfield_graph.laplacian_matrix();
    println!("Laplacian matrix:");
    for row in laplacian {
        println!("{:?}", row);
    }

    println!("----");
}
