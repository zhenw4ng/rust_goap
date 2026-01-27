use crate::basic::mutation::Mutation;
use std::hash::{Hash, Hasher};

/// The effect is what happens when an Action is applied.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Effect {
    /// The [`Mutator`] that are active when this effect is applied.
    pub mutations: Vec<Mutation>,
    /// The cost of applying this effect. Default is 1.
    pub cost: usize,
}

impl Default for Effect {
    fn default() -> Self {
        Self {
            mutations: vec![],
            cost: 1,
        }
    }
}

impl Effect {
    /// Creates a new effect with the given action name.
    pub fn new() -> Self {
        Self {
            mutations: vec![],
            cost: 1,
        }
    }
}

impl Hash for Effect {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.mutations.hash(state);
        self.cost.hash(state);
    }
}
