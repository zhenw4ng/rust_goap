use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

use crate::basic::value::Value;
use crate::goal::Goal;

/// Represents the current state of the world in a Goal-Oriented Action Planning (GOAP) system.
///
/// `WorldState` is the fundamental data structure that tracks all relevant information
/// about the game world or simulation environment. It serves as both the starting point
/// for planning and the evolving state that actions transform during plan execution.
///
/// The world state is implemented as a map from string keys to typed values (`Value`),
/// allowing for flexible representation of various game state variables such as:
/// - Boolean flags: `"is_hungry"`, `"has_weapon"`, `"door_unlocked"`
/// - Numerical quantities: `"health"`, `"ammo_count"`, `"gold_amount"`
/// - Continuous values: `"distance_to_target"`, `"time_remaining"`
///
/// # Key Characteristics
/// - **Immutable by default**: Methods return new instances to support functional style
/// - **Deterministic**: Same inputs always produce same world state
/// - **Comparable**: Can be compared for equality and hashed
/// - **Serializable**: Can be converted to/from string representations
///
/// # Usage Example
/// ```
/// use goap_lite::prelude::*;
///
/// // Create a simple world state for a character
/// let character_state = WorldState::new()
///     .set("health", 100)
///     .set("is_hungry", true)
///     .set("has_weapon", false)
///     .set("ammo_count", 0);
///
/// // Create a more complex game state
/// let game_state = WorldState::new()
///     .set("player_health", 75.0)
///     .set("enemy_count", 3)
///     .set("time_of_day", 14.5)  // 2:30 PM
///     .set("mission_complete", false);
/// ```
#[derive(Debug, Clone, Eq, PartialEq, Default)]
pub struct WorldState(pub(super) BTreeMap<String, Value>);

impl WorldState {
    /// Creates a new empty world state.
    ///
    /// Returns a `WorldState` with no variables defined. This is typically used
    /// as the starting point for building up a complete world state using the
    /// `set` method.
    ///
    /// # Returns
    /// A new empty `WorldState` instance.
    ///
    /// # Example
    /// ```
    /// use goap_lite::prelude::*;
    ///
    /// let empty_state = WorldState::new();
    /// // The state is truly empty - no variables defined
    /// ```
    pub fn new() -> Self {
        Self(BTreeMap::new())
    }

    /// Sets or updates a variable in the world state.
    ///
    /// This method uses the builder pattern, returning a new `WorldState` with
    /// the specified variable set to the given value. If the variable already
    /// exists, its value is overwritten.
    ///
    /// # Arguments
    /// * `key` - The name of the variable to set
    /// * `value` - The value to assign (any type that implements `Into<Value>`)
    ///
    /// # Returns
    /// A new `WorldState` instance with the variable set (for method chaining).
    ///
    /// # Example
    /// ```
    /// use goap_lite::prelude::*;
    ///
    /// // Build a world state incrementally
    /// let state = WorldState::new()
    ///     .set("health", 100)
    ///     .set("stamina", 75.0)
    ///     .set("has_key", true);
    ///
    /// // Update an existing variable
    /// let updated_state = state.set("health", 85);  // Health reduced to 85
    /// ```
    pub fn set(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.0.insert(key.into(), value.into());
        self
    }

    /// Calculates the heuristic distance from this world state to a goal.
    ///
    /// This method is used by the A* planning algorithm to estimate how far
    /// the current state is from satisfying all requirements of a given goal.
    /// The distance is calculated as the sum of individual distances between
    /// each goal requirement and the corresponding world state value.
    ///
    /// # Distance Calculation
    /// - For each goal requirement, find the corresponding world state value
    /// - If the value exists: calculate type-specific distance (see `Value::distance`)
    /// - If the value doesn't exist: apply a penalty of 1
    /// - Sum all distances to get total heuristic distance
    ///
    /// # Arguments
    /// * `goal` - The goal to measure distance to
    ///
    /// # Returns
    /// A `u64` representing the total heuristic distance to the goal.
    /// Lower values indicate states closer to satisfying the goal.
    ///
    /// # Example
    /// ```
    /// use goap_lite::prelude::*;
    ///
    /// // Create a world state
    /// let state = WorldState::new()
    ///     .set("health", 60)
    ///     .set("has_weapon", true)
    ///     .set("ammo", 5);
    ///
    /// // Create a goal
    /// let goal = Goal::new()
    ///     .with("health", Assert::gt_eq(80))   // Want health >= 80
    ///     .with("has_weapon", Assert::eq(true)) // Must have weapon
    ///     .with("ammo", Assert::gt_eq(10));     // Want ammo >= 10
    ///
    /// // Calculate distance (health: 20 away, weapon: satisfied, ammo: 5 away)
    /// let distance = state.distance_to_goal(&goal);
    /// // distance = 20 (health) + 0 (weapon) + 5 (ammo) = 25
    /// ```
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

    /// Retrieves the value of a variable from the world state.
    ///
    /// # Arguments
    /// * `key` - The name of the variable to retrieve
    ///
    /// # Returns
    /// `Some(&Value)` if the variable exists, `None` otherwise.
    ///
    /// # Example
    /// ```
    /// use goap_lite::prelude::*;
    ///
    /// let state = WorldState::new()
    ///     .set("health", 100)
    ///     .set("name", "player");
    ///
    /// let health = state.get("health");
    /// assert!(health.is_some());
    ///
    /// let missing = state.get("non_existent");
    /// assert!(missing.is_none());
    /// ```
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.0.get(key)
    }

    /// Checks if a variable exists in the world state.
    ///
    /// # Arguments
    /// * `key` - The name of the variable to check
    ///
    /// # Returns
    /// `true` if the variable exists, `false` otherwise.
    ///
    /// # Example
    /// ```
    /// use goap_lite::prelude::*;
    ///
    /// let state = WorldState::new()
    ///     .set("health", 100)
    ///     .set("stamina", 75);
    ///
    /// assert!(state.contains_key("health"));
    /// assert!(!state.contains_key("mana"));
    /// ```
    pub fn contains_key(&self, key: &str) -> bool {
        self.0.contains_key(key)
    }

    /// Returns the number of variables in the world state.
    ///
    /// # Returns
    /// The count of variables defined in this world state.
    ///
    /// # Example
    /// ```
    /// use goap_lite::prelude::*;
    ///
    /// let state = WorldState::new()
    ///     .set("a", 1)
    ///     .set("b", 2)
    ///     .set("c", 3);
    ///
    /// assert_eq!(state.len(), 3);
    /// ```
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Checks if the world state is empty (contains no variables).
    ///
    /// # Returns
    /// `true` if no variables are defined, `false` otherwise.
    ///
    /// # Example
    /// ```
    /// use goap_lite::prelude::*;
    ///
    /// let empty_state = WorldState::new();
    /// let populated_state = WorldState::new().set("key", 42);
    ///
    /// assert!(empty_state.is_empty());
    /// assert!(!populated_state.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns an iterator over all variables in the world state.
    ///
    /// The iterator yields `(&String, &Value)` pairs in alphabetical order
    /// by key (due to using `BTreeMap` internally).
    ///
    /// # Returns
    /// An iterator over the world state's variables.
    ///
    /// # Example
    /// ```
    /// use goap_lite::prelude::*;
    ///
    /// let state = WorldState::new()
    ///     .set("health", 100)
    ///     .set("ammo", 50)
    ///     .set("stamina", 75.0);
    ///
    /// let mut variables = Vec::new();
    /// for (key, value) in state.iter() {
    ///     variables.push((key.clone(), *value));
    /// }
    ///
    /// // Variables are iterated in alphabetical order: ammo, health, stamina
    /// assert_eq!(variables.len(), 3);
    /// ```
    pub fn iter(&self) -> std::collections::btree_map::Iter<'_, String, Value> {
        self.0.iter()
    }
}

impl Hash for WorldState {
    /// Computes a hash value for the world state.
    ///
    /// The hash includes:
    /// 1. The number of variables (for quick differentiation of different-sized states)
    /// 2. Each key-value pair (ensuring different variable sets hash differently)
    ///
    /// This enables using `WorldState` instances as keys in hash-based collections,
    /// which is crucial for the planning algorithm's state caching and duplicate detection.
    ///
    /// # Hash Consistency
    /// - The same world state always hashes to the same value
    /// - Different world states (with different variables/values) hash to different values
    /// - Order of insertion doesn't affect the hash (due to `BTreeMap` sorting)
    ///
    /// # Example
    /// ```
    /// use std::collections::hash_map::DefaultHasher;
    /// use std::hash::{Hash, Hasher};
    /// use goap_lite::prelude::*;
    ///
    /// let state1 = WorldState::new()
    ///     .set("health", 100)
    ///     .set("ammo", 50);
    ///
    /// let state2 = WorldState::new()
    ///     .set("ammo", 50)  // Same variables, different order
    ///     .set("health", 100);
    ///
    /// let state3 = WorldState::new()
    ///     .set("health", 75)  // Different value
    ///     .set("ammo", 50);
    ///
    /// let mut hasher1 = DefaultHasher::new();
    /// let mut hasher2 = DefaultHasher::new();
    /// let mut hasher3 = DefaultHasher::new();
    ///
    /// state1.hash(&mut hasher1);
    /// state2.hash(&mut hasher2);
    /// state3.hash(&mut hasher3);
    ///
    /// assert_eq!(hasher1.finish(), hasher2.finish()); // Same content
    /// assert_ne!(hasher1.finish(), hasher3.finish()); // Different content
    /// ```
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.len().hash(state);
        for (key, value) in &self.0 {
            key.hash(state);
            value.hash(state);
        }
    }
}

/// Provides a convenient way to create world states from key-value pairs.
///
/// This trait allows creating `WorldState` instances using the `into()` method
/// on tuples or vectors of key-value pairs.
///
/// # Example
/// ```
/// use goap_lite::prelude::*;
///
/// // Create from a vector of tuples
/// let state: WorldState = vec![
///     ("health".to_string(), Value::I64(100)),
///     ("ammo".to_string(), Value::I64(50)),
/// ].into();
///
/// assert_eq!(state.len(), 2);
/// ```
impl From<Vec<(String, Value)>> for WorldState {
    fn from(pairs: Vec<(String, Value)>) -> Self {
        let mut state = WorldState::new();
        for (key, value) in pairs {
            state = state.set(key, value);
        }
        state
    }
}

/// Provides string representation for debugging and display purposes.
///
/// Formats the world state as a readable string showing all variables
/// and their values.
///
/// # Example
/// ```
/// use goap_lite::prelude::*;
/// use std::fmt::Write;
///
/// let state = WorldState::new()
///     .set("health", 100)
///     .set("name", "hero")
///     .set("active", true);
///
/// let mut output = String::new();
/// write!(&mut output, "{}", state).unwrap();
///
/// // Output format: WorldState { health: Value:I64(100), name: Value:Bool(true), active: Value:Bool(true) }
/// assert!(output.contains("health"));
/// assert!(output.contains("name"));
/// assert!(output.contains("active"));
/// ```
impl std::fmt::Display for WorldState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WorldState {{ ")?;
        let mut first = true;
        for (key, value) in &self.0 {
            if !first {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", key, value)?;
            first = false;
        }
        write!(f, " }}")
    }
}
