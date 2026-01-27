use std::hash::{Hash, Hasher};

use std::collections::BTreeMap;

use crate::basic::value::Value;
use crate::goal::Goal;

/// This is our internal state that the planner uses to progress in the path finding,
/// until we reach our [`Goal`]
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct WorldState(pub(super) BTreeMap<String, Value>);

impl WorldState {
    /// Create a new empty state
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    pub fn set(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.0.insert(key.into(), value.into());
        self
    }

    /// The total distance to the goal in terms of differences between the goal's requirements and the local state's data
    pub fn distance_to_goal(&self, goal: &Goal) -> u64 {
        goal.requirements
            .iter()
            .map(|(key, goal_val)| {
                match self.0.get(key) {
                    Some(state_val) => state_val.distance(&goal_val.value()),
                    None => 1, // Penalty for missing keys
                }
            })
            .sum()
    }
}

impl Hash for WorldState {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.len().hash(state);
        for (key, value) in &self.0 {
            key.hash(state);
            value.hash(state);
        }
    }
}
