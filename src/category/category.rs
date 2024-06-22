use std::fmt;

pub struct Category {
    pub objects: Vec<String>,
    pub morphisms: Vec<Morphism>,
}

#[derive(PartialEq, Clone, Debug)]
pub struct Morphism {
    pub source: String,
    pub target: String,
    pub is_isomorphism: bool,
}

impl Category {
    pub fn new() -> Self {
        Category {
            objects: Vec::new(),
            morphisms: Vec::new(),
        }
    }

    pub fn add_object(&mut self, object: &str) {
        self.objects.push(object.to_string());
    }

    pub fn add_morphism(&mut self, source: &str, target: &str, is_isomorphism: bool) {
        self.morphisms.push(Morphism {
            source: source.to_string(),
            target: target.to_string(),
            is_isomorphism,
        });
    }

    pub fn get_isomorphisms(&self) -> Vec<&Morphism> {
        self.morphisms.iter().filter(|m| m.is_isomorphism).collect()
    }
}

impl fmt::Display for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Category:\n Objects: {:?}\n Morphisms:\n", self.objects)?;
        for morphism in &self.morphisms {
            write!(
                f,
                "  {} -> {} (isomorphism: {})\n",
                morphism.source, morphism.target, morphism.is_isomorphism
            )?;
        }
        Ok(())
    }
}
