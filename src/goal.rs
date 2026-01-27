use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

use crate::basic::assert::Assert;

/// Goal is a map of what we want our final [`WorldState`](crate::world_state::WorldState) to be, using String as
/// keys and [`Evaluate`] to assert what we want the [`Value`](crate::value::Value) to be
#[derive(Clone, Debug, PartialEq)]
pub struct Goal {
    /// The goal state of the world
    /// some value equals, some value not equals, ...
    pub requirements: BTreeMap<String, Assert>,
}

impl Hash for Goal {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.requirements.len().hash(state);
        for (key, value) in &self.requirements {
            key.hash(state);
            value.hash(state);
        }
    }
}

impl Default for Goal {
    fn default() -> Self {
        Self::new()
    }
}

impl Goal {
    /// Create a new empty goal
    pub fn new() -> Self {
        Self {
            requirements: BTreeMap::new(),
        }
    }

    /// Create a new goal with a single requirements
    pub fn with(mut self, key: impl Into<String>, evalute: impl Into<Assert>) -> Self {
        self.requirements.insert(key.into(), evalute.into());
        self
    }

    /// Create a new goal from a list of requirements
    pub fn from_reqs(conditions: &[(String, Assert)]) -> Self {
        let mut goal = Goal::new();
        for (k, v) in conditions {
            goal = goal.with(k, v.clone());
        }
        goal
    }
}
