use crate::graph::graph::Graph;
use crate::graph::hopfield::HopfieldNode;
use crate::graph::node::Node;
use crate::sheaf::conditions::SheafCondition;
use colored::*;

pub fn simulate_hopfield_network(
    graph: &mut Graph<HopfieldNode>,
    max_iterations: usize,
    condition: &dyn SheafCondition,
) {
    println!("Starting Hopfield network simulation...");
    let mut iteration = 0;
    let print_interval = max_iterations / 10;
    let verbose = false;

    while iteration < max_iterations {
        let mut updates = Vec::new();

        // Collect necessary data about neighbors first
        let neighbor_states: Vec<(usize, Vec<(usize, f64)>)> = graph
            .nodes
            .iter()
            .map(|node| {
                let neighbors: Vec<_> = node
                    .get_neighbors()
                    .iter()
                    .map(|&neighbor_id| {
                        let neighbor_state = graph.get_node(neighbor_id).unwrap().state as f64;
                        (neighbor_id, neighbor_state)
                    })
                    .collect();
                (node.get_id(), neighbors)
            })
            .collect();

        for node in &mut graph.nodes {
            if let Some((_, neighbors)) =
                neighbor_states.iter().find(|(id, _)| *id == node.get_id())
            {
                let mut sum = 0.0;
                for (i, (_, neighbor_state)) in neighbors.iter().enumerate() {
                    sum += neighbor_state * node.weights[i];
                }
                node.state = update_hopfield_node_state(sum);
                node.set_info(format!("State: {}", node.state));
                updates.push((node.get_name().to_string(), node.get_info().clone()));
            }
        }

        for (name, info) in updates {
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
        } else {
            println!(
                "{}",
                format!("Sheaf condition satisfied after iteration {}.", iteration).green()
            );
        }

        // Check for convergence (optional, depending on your model)
        let converged = graph.nodes.iter().all(|node| {
            node.state
                == update_hopfield_node_state(
                    node.get_neighbors()
                        .iter()
                        .map(|&neighbor_id| {
                            let neighbor = graph.get_node(neighbor_id).unwrap();
                            neighbor.state as f64
                        })
                        .sum::<f64>(),
                )
        });
        if converged {
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
    //         "Sheaf condition satisfied after chip firing simulation.".green()
    //     );
    // } else {
    //     println!(
    //         "{}",
    //         "Sheaf condition violated after chip firing simulation.".red()
    //     );
    // }
}

fn update_hopfield_node_state(sum: f64) -> i32 {
    if sum > 0.0 {
        1
    } else {
        -1
    }
}
