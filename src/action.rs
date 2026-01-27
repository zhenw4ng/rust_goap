use std::hash::{Hash, Hasher};

use crate::basic::assert::{Assert, compare_values};
use crate::effect::Effect;
use crate::world_state::WorldState;

/// Represents an executable action in a Goal-Oriented Action Planning (GOAP) system.
///
/// An `Action` defines something that an entity can perform, provided that all
/// preconditions are satisfied in the current world state. When executed, the
/// action produces effects that modify the world state.
///
/// # Key Components
/// - **Key**: A unique identifier for the action (e.g., "Attack", "GatherWood")
/// - **Preconditions**: Conditions that must be true before the action can be executed
/// - **Effect**: The changes to the world state that result from executing the action
///
/// # Usage Example
/// ```
/// use goap_lite::prelude::*;
///
/// // Create an action for attacking an enemy
/// let attack_action = Action::new("Attack")
///     .with_precondition(("has_weapon", Assert::eq(true)))
///     .with_precondition(("enemy_in_range", Assert::eq(true)))
///     .with_effect(Effect::new().with_mutation("enemy_health", Mutation::decrease(10)));
///
/// // Create an action for gathering resources
/// let gather_action = Action::new("GatherWood")
///     .with_precondition(("has_axe", Assert::eq(true)))
///     .with_precondition(("near_forest", Assert::eq(true)))
///     .with_effect(Effect::new().with_mutation("wood_count", Mutation::increase(1)));
/// ```
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Action {
    /// Unique identifier for this action.
    ///
    /// Used to reference the action in plans and debugging output.
    /// Should be descriptive (e.g., "Attack", "Heal", "GatherResources").
    pub key: String,

    /// Conditions that must be satisfied before this action can be executed.
    ///
    /// Each precondition is a tuple of (key, assertion) where:
    /// - `key`: The world state variable to check
    /// - `assertion`: The condition that must be true for that variable
    ///
    /// All preconditions must be satisfied simultaneously for the action to be valid.
    pub preconditions: Vec<(String, Assert)>,

    /// The outcome of executing this action.
    ///
    /// Contains mutations that will be applied to the world state when the
    /// action is successfully executed. If `None`, the action has no effect
    /// on the world state (though it may still have other purposes).
    pub effect: Option<Effect>,
}

impl Hash for Action {
    /// Computes a hash value for the action.
    ///
    /// The hash includes the action key, all preconditions, and the effect.
    /// This enables using `Action` instances as keys in hash-based collections.
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.key.hash(state);
        self.preconditions.hash(state);
        self.effect.hash(state);
    }
}

impl Action {
    /// Creates a new action with the given identifier.
    ///
    /// # Arguments
    /// * `key` - A unique identifier for the action. Can be any type that
    ///           implements `Into<String>`.
    ///
    /// # Returns
    /// A new `Action` instance with no preconditions and no effect.
    ///
    /// # Examples
    /// ```
    /// use goap_lite::prelude::*;
    ///
    /// let action = Action::new("MoveToTarget");
    /// assert_eq!(action.key, "MoveToTarget");
    /// assert!(action.preconditions.is_empty());
    /// assert!(action.effect.is_none());
    /// ```
    pub fn new(key: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            preconditions: vec![],
            effect: None,
        }
    }

    /// Adds a precondition to the action.
    ///
    /// This method uses the builder pattern, allowing for fluent chaining.
    ///
    /// # Arguments
    /// * `(key, compare)` - A tuple containing:
    ///   - `key`: The world state variable to check
    ///   - `compare`: The assertion that must be true for that variable
    ///
    /// # Returns
    /// The modified `Action` instance (for method chaining).
    ///
    /// # Examples
    /// ```
    /// use goap_lite::prelude::*;
    ///
    /// let action = Action::new("Attack")
    ///     .with_precondition(("has_weapon", Assert::eq(true)))
    ///     .with_precondition(("ammo_count", Assert::gt(0)));
    ///
    /// assert_eq!(action.preconditions.len(), 2);
    /// ```
    pub fn with_precondition(mut self, (key, compare): (impl Into<String>, Assert)) -> Self {
        self.preconditions.push((key.into(), compare));
        self
    }

    /// Sets the effect that occurs when this action is executed.
    ///
    /// This method uses the builder pattern, allowing for fluent chaining.
    ///
    /// # Arguments
    /// * `effect` - The `Effect` that will be applied to the world state
    ///              when this action is executed.
    ///
    /// # Returns
    /// The modified `Action` instance (for method chaining).
    ///
    /// # Examples
    /// ```
    /// use goap_lite::prelude::*;
    ///
    /// let effect = Effect::new().with_mutation("enemy_health", Mutation::decrease(10));
    /// let action = Action::new("Attack").with_effect(effect);
    ///
    /// assert!(action.effect.is_some());
    /// ```
    pub fn with_effect(mut self, effect: Effect) -> Self {
        self.effect = Some(effect);
        self
    }

    /// Checks whether all preconditions are satisfied in the given world state.
    ///
    /// This is the core validation function that determines if an action
    /// can be executed in the current context.
    ///
    /// # Arguments
    /// * `world_state` - The current state of the world to check against
    ///
    /// # Returns
    /// `true` if all preconditions are satisfied, `false` otherwise.
    ///
    /// # Panics
    /// Panics if a precondition references a world state variable that doesn't exist.
    /// This is a deliberate design choice to catch configuration errors early.
    ///
    /// # Examples
    /// ```
    /// use goap_lite::prelude::*;
    ///
    /// // Create a world state
    /// let world_state = WorldState::new()
    ///     .set("has_weapon", true)
    ///     .set("ammo_count", 5)
    ///     .set("enemy_in_range", true);
    ///
    /// // Create an action with preconditions
    /// let action = Action::new("Attack")
    ///     .with_precondition(("has_weapon", Assert::eq(true)))
    ///     .with_precondition(("ammo_count", Assert::gt(0)))
    ///     .with_precondition(("enemy_in_range", Assert::eq(true)));
    ///
    /// // Check if the action can be executed
    /// assert!(action.check_preconditions(&world_state));
    ///
    /// // Modify the world state to violate a precondition
    /// let bad_state = WorldState::new()
    ///     .set("has_weapon", true)
    ///     .set("ammo_count", 0)  // No ammo!
    ///     .set("enemy_in_range", true);
    ///
    /// assert!(!action.check_preconditions(&bad_state));
    /// ```
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
