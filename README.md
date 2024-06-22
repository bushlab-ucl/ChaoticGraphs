# `ChaoticNetworks`

## Overview

`ChaoticNetworks` is a Rust package designed for advanced graph modeling and simulations. This library leverages `Grothendieck topologies` and `sheaf theory` to provide a robust framework for analyzing and understanding complex networks such as `Hopfield networks` and `chip-firing models`.

## Features
- `Modular design` and support for a range of `Grothendieck Topologies`.
- Integration with `sheaf theory` for consistency and advanced analysis.
- Flexible simulation framework for `Hopfield networks` and `chip-firing systems`.
- `Rust` implementation (potential for future `Python` bindings).


## Libraries
- `src/graph` - Contains the core graph structures and node implementations.
- `src/sheaf` - Manages sheaf and presheaf data, along with sheaf conditions.
- `src/simulation` - Simulation logic for Hopfield networks and chip-firing systems.
- `src/category` - Defines Category Theoeretic structure and Grothendieck topologies.

## Links
- `UCL Human Electrophysiology Lab`: https://bushlab-ucl.github.io
- `Crates.io`: https://crates.io/crates/chaotic-networks
- `SGA 1972`: [https://stacks.math.columbia.edu/tag/00WY](https://stacks.math.columbia.edu/)
  
# Structure and Use of ChaoticGraphs in Rust

### Documentation
This README may be out of date, check out the docs at:
**Crates.io**: https://crates.io/crates/chaotic-networks

### Rust Module Overview

The main modules include:

- graph: Contains core graph structures and nodes.
- sheaf: Manages sheaf and presheaf data structures and conditions.
- simulation: Contains simulation logic for various graph models.
- topology: Defines and manages Grothendieck topologies.
utils: Additional utilities and helper functions.

## Graph Structures

The `Graph` struct is the core of the ChaoticGraphs library. It supports adding nodes, edges, and managing topologies and sheaves.

1. **Graph Struct**: Graph Configuration

```rust
Copy code
pub struct Graph<T: Node> {
    pub nodes: Vec<T>,
    pub category: Category,
    pub topology: GrothendieckTopology,
    pub sheaf: Sheaf,
}
```

Topologies
Topologies define the structure and properties of the graph.

GrothendieckTopology Struct
rust
Copy code
#[derive(Clone, Debug)]
pub struct GrothendieckTopology {
    pub coverings: Vec<Covering>,
}

impl GrothendieckTopology {
    pub fn new() -> Self { ... }
    pub fn add_covering(&mut self, covering: Covering) { ... }
    pub fn is_covering(&self, morphism: &Morphism) -> bool { ... }
    pub fn check_covering(&self, _object: &str, morphisms: &[Morphism]) -> bool { ... }
    pub fn indiscrete(objects: Vec<String>) -> Self { ... }
    pub fn discrete(objects: Vec<String>) -> Self { ... }
}
Sheaf Management
Sheaf management ensures local-to-global consistency in the graph.

Sheaf Struct
rust
Copy code
pub struct Sheaf {
    pub presheaf: Presheaf,
    pub topology: GrothendieckTopology,
}

impl Sheaf {
    pub fn new(topology: GrothendieckTopology) -> Self { ... }
    pub fn assign(&mut self, object: &str, value: Vec<String>) { ... }
    pub fn get(&self, object: &str) -> Option<&Vec<String>> { ... }
    pub fn check_sheaf_condition(&self) -> bool { ... }
}
Simulations
Simulations process the graph nodes and apply specific logic like Hopfield networks or chip-firing systems.

simulate_chip_firing
rust
Copy code
pub fn simulate_chip_firing(graph: &mut Graph<ChipFiringNode>, max_iterations: usize, condition: &dyn SheafCondition) {
    println!("Starting chip firing simulation...");
    // Simulation logic
    if graph.check_sheaf_condition() {
        println!("{}", "Sheaf condition satisfied after chip firing simulation.".green());
    } else {
        println!("{}", "Sheaf condition violated after chip firing simulation.".red());
    }
}
simulate_hopfield_network
rust
Copy code
pub fn simulate_hopfield_network(graph: &mut Graph<HopfieldNode>, max_iterations: usize, condition: &dyn SheafCondition) {
    println!("Starting Hopfield network simulation...");
    // Simulation logic
    if graph.check_sheaf_condition() {
        println!("{}", "Sheaf condition satisfied after Hopfield network simulation.".green());
    } else {
        println!("{}", "Sheaf condition violated after Hopfield network simulation.".red());
    }
}
