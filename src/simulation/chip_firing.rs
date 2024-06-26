use crate::graph::chip_firing::ChipFiringNode;
use crate::graph::graph::Graph;
use crate::graph::node::Node;
use crate::sheaf::conditions::SheafCondition;
use colored::*;

pub fn simulate_chip_firing(
    graph: &mut Graph<ChipFiringNode>,
    max_iterations: usize,
    condition: &dyn SheafCondition,
) {
    println!("Starting chip firing simulation...");
    let mut any_fired = true;
    let mut iteration = 0;
    let print_interval = max_iterations / 10;
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

        if !condition.check(&graph.sheaf) {
            println!(
                "{}",
                format!("Sheaf condition violated after iteration {}.", iteration).red()
            );
            break;
        }
    }

    if iteration >= max_iterations {
        println!("Reached maximum iterations.");
    }

    println!("{}", graph);

    // if graph.check_global_sheaf_consistency() {
    //     println!(
    //         "{}",
    //         "Global Sheaf condition satisfied after chip firing simulation.".green()
    //     );
    // } else {
    //     println!(
    //         "{}",
    //         "Sheaf condition violated after chip firing simulation.".red()
    //     );
    // }
}
