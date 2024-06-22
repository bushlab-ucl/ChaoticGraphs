use crate::category::category::Morphism;
use std::fmt;

#[derive(Clone, Debug)]
pub struct Covering {
    pub covers: Vec<Morphism>,
}

#[derive(Clone)]
pub struct GrothendieckTopology {
    pub coverings: Vec<Covering>,
}

impl GrothendieckTopology {
    pub fn new() -> Self {
        GrothendieckTopology {
            coverings: Vec::new(),
        }
    }

    pub fn add_covering(&mut self, covering: Covering) {
        self.coverings.push(covering);
    }

    pub fn is_covering(&self, morphism: &Morphism) -> bool {
        for covering in &self.coverings {
            if covering.covers.contains(morphism) {
                return true;
            }
        }
        false
    }

    // Check if a given set of morphisms form a covering for a given object
    pub fn check_covering(&self, _object: &str, morphisms: &[Morphism]) -> bool {
        for covering in &self.coverings {
            let mut all_covered = true;
            for morphism in &covering.covers {
                if !morphisms.contains(morphism) {
                    all_covered = false;
                    break;
                }
            }
            if all_covered {
                return true;
            }
        }
        false
    }

    // Specific method to initialize an indiscrete topology
    pub fn indiscrete(objects: Vec<String>) -> Self {
        let mut topology = GrothendieckTopology::new();
        for object in objects {
            let identity_morphism = Morphism {
                source: object.clone(),
                target: object.clone(),
                is_isomorphism: true,
            };
            topology.add_covering(Covering {
                covers: vec![identity_morphism],
            });
        }
        topology
    }

    // Specific method to initialize a discrete topology
    pub fn discrete(objects: Vec<String>) -> Self {
        let mut topology = GrothendieckTopology::new();
        for object in &objects {
            let identity_morphism = Morphism {
                source: object.clone(),
                target: object.clone(),
                is_isomorphism: true,
            };
            topology.add_covering(Covering {
                covers: vec![identity_morphism],
            });
        }

        // In a discrete topology, every morphism between any two objects is considered
        for source in &objects {
            for target in &objects {
                let morphism = Morphism {
                    source: source.clone(),
                    target: target.clone(),
                    is_isomorphism: true,
                };
                topology.add_covering(Covering {
                    covers: vec![morphism],
                });
            }
        }

        topology
    }
}

impl fmt::Display for GrothendieckTopology {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "GrothendieckTopology:\n Coverings:\n")?;
        for covering in &self.coverings {
            write!(f, "  Covering:\n")?;
            for morphism in &covering.covers {
                write!(
                    f,
                    "    {} -> {} (isomorphism: {})\n",
                    morphism.source, morphism.target, morphism.is_isomorphism
                )?;
            }
        }
        Ok(())
    }
}
