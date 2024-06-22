use crate::graph::chip_firing::ChipFiringNode;
use crate::graph::graph::Graph;
use crate::graph::node::Node;
use crate::utils::conditions::check_sum_condition;

pub fn simulate_chip_firing(graph: &mut Graph<ChipFiringNode>, max_iterations: usize) {
    println!("Starting chip firing simulation...");
    let mut any_fired = true;
    let mut iteration = 0;
    let print_interval = max_iterations / 10;
    let threshold = 10;
    let verbose = false;

    while any_fired && iteration < max_iterations {
        any_fired = false;
        let mut updates = Vec::new();
        for node in &mut graph.nodes {
            while node.num_chips >= node.get_neighbors().len() as u32 {
                any_fired = true;
                node.num_chips -= node.get_neighbors().len() as u32;
                for neighbor_id in node.get_neighbors() {
                    updates.push(neighbor_id);
                }
            }
            node.set_info(format!("Chips: {}", node.num_chips));
            graph.sheaf.assign(node.get_name(), vec![node.get_info()]);
        }

        let mut neighbor_updates = Vec::new();
        for neighbor_id in updates {
            graph.update_node(neighbor_id, |neighbor| {
                neighbor.num_chips += 1;
                neighbor.set_info(format!("Chips: {}", neighbor.num_chips));
            });
            let neighbor_info = {
                let neighbor = graph.get_node(neighbor_id).unwrap();
                (neighbor.get_name().to_string(), neighbor.get_info())
            };
            neighbor_updates.push(neighbor_info);
        }

        for (name, info) in neighbor_updates {
            graph.sheaf.assign(&name, vec![info]);
        }

        iteration += 1;
        if verbose && iteration % print_interval == 0 {
            println!("Iteration: {}", iteration);
        }

        if !check_sum_condition(&graph.sheaf, threshold) {
            println!("Sum condition violated after iteration {}.", iteration);
            break;
        }
    }

    if iteration >= max_iterations {
        println!("Reached maximum iterations.");
    }

    if graph.check_sheaf_condition() {
        println!("Sheaf condition satisfied after chip firing simulation.");
    } else {
        println!("Sheaf condition violated after chip firing simulation.");
    }
}
