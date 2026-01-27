use std::hash::{Hash, Hasher};

use crate::basic::assert::{Assert, compare_values};
use crate::effect::Effect;
use crate::world_state::WorldState;

/// An `Action` represents something your Entity can do, granted the `LocalState`
/// is as defined in the `preconditions`. It has a list of `Effect`'s that apply
/// if the NPC successfully executed the task.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Action {
    pub key: String,
    /// What preconditions need to be true before we can execute this action
    pub preconditions: Vec<(String, Assert)>,
    /// What is the outcome from doing this action
    pub effect: Option<Effect>,
}

impl Hash for Action {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
        self.preconditions.hash(state);
        self.effect.hash(state);
    }
}

impl Action {
    /// Create a new action with the given key.
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            preconditions: vec![],
            effect: None,
        }
    }

    pub fn with_precondition(mut self, (key, compare): (impl Into<String>, Assert)) -> Self {
        self.preconditions.push((key.into(), compare));
        self
    }

    pub fn with_effect(mut self, effect: Effect) -> Self {
        self.effect = Some(effect);
        self
    }

    pub fn check_preconditions(&self, world_state: &WorldState) -> bool {
        self.preconditions.iter().all(|(key, compare)| {
            let state_value = world_state
                .0
                .get(key)
                .unwrap_or_else(|| panic!("Couldn't find key {key:#?} in WorldState"));
            compare_values(compare, state_value)
        })
    }
}
