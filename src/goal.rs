use std::collections::BTreeMap;
use std::hash::{Hash, Hasher};

use crate::basic::assert::Assert;

/// Represents a desired state of the world in a Goal-Oriented Action Planning (GOAP) system.
///
/// A `Goal` defines the target conditions that the planner attempts to achieve by
/// finding a sequence of actions. It consists of a set of requirements, where each
/// requirement specifies a world state variable and the condition it must satisfy.
///
/// Goals are the driving force behind the planning process - they define what
/// "success" looks like for the planning algorithm. The planner searches for
/// action sequences that transform the current world state into one that satisfies
/// all goal requirements.
///
/// # Key Concepts
/// - **Requirements**: A map of world state variables to assertions that must be true
/// - **Assertions**: Conditions like equality, inequality, or comparisons (>, <, >=, <=)
/// - **Partial Satisfaction**: Goals can have multiple requirements; all must be satisfied
///
/// # Usage Example
/// ```
/// use rust_goap::prelude::*;
///
/// // Create a simple goal: character should not be hungry
/// let simple_goal = Goal::new().with("is_hungry", Assert::eq(false));
///
/// // Create a complex goal with multiple requirements
/// let complex_goal = Goal::new()
///     .with("health", Assert::gt_eq(80))      // Health must be at least 80
///     .with("has_weapon", Assert::eq(true))   // Must have a weapon
///     .with("enemy_count", Assert::eq(0));    // No enemies remaining
///
/// // Create a goal from a list of requirements
/// let requirements = vec![
///     ("gold".to_string(), Assert::gt_eq(100)),
///     ("food".to_string(), Assert::gt_eq(10)),
/// ];
/// let from_list_goal = Goal::from_reqs(&requirements);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Goal {
    /// The requirements that define this goal.
    ///
    /// Each entry in this map represents a condition that must be satisfied
    /// for the goal to be considered achieved. The key is a world state
    /// variable name, and the value is an assertion that must hold true
    /// for that variable.
    ///
    /// All requirements must be satisfied simultaneously. If any requirement
    /// is not met, the goal is not achieved.
    ///
    /// # Example
    /// ```
    /// use rust_goap::prelude::*;
    ///
    /// let goal = Goal::new()
    ///     .with("health", Assert::gt_eq(50))
    ///     .with("ammo", Assert::gt(0));
    ///
    /// // The goal has 2 requirements
    /// assert_eq!(goal.requirements.len(), 2);
    ///
    /// // Check specific requirements
    /// let health_req = goal.requirements.get("health");
    /// assert!(health_req.is_some());
    /// assert!(matches!(health_req.unwrap(), Assert::GreaterThanEquals(_)));
    /// ```
    pub requirements: BTreeMap<String, Assert>,
}

impl Hash for Goal {
    /// Computes a hash value for the goal.
    ///
    /// The hash includes the number of requirements and each individual
    /// requirement (key-value pair). This ensures that goals with different
    /// requirements hash to different values.
    ///
    /// This enables using `Goal` instances as keys in hash-based collections,
    /// which is useful for caching planning results or detecting duplicate goals.
    ///
    /// # Example
    /// ```
    /// use std::collections::hash_map::DefaultHasher;
    /// use std::hash::{Hash, Hasher};
    /// use rust_goap::prelude::*;
    ///
    /// let goal1 = Goal::new().with("health", Assert::gt_eq(50));
    /// let goal2 = Goal::new().with("health", Assert::gt_eq(50));
    /// let goal3 = Goal::new().with("health", Assert::gt_eq(100));
    ///
    /// let mut hasher1 = DefaultHasher::new();
    /// let mut hasher2 = DefaultHasher::new();
    /// let mut hasher3 = DefaultHasher::new();
    ///
    /// goal1.hash(&mut hasher1);
    /// goal2.hash(&mut hasher2);
    /// goal3.hash(&mut hasher3);
    ///
    /// assert_eq!(hasher1.finish(), hasher2.finish()); // Same requirements
    /// assert_ne!(hasher1.finish(), hasher3.finish()); // Different requirements
    /// ```
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.requirements.len().hash(state);
        for (key, value) in &self.requirements {
            key.hash(state);
            value.hash(state);
        }
    }
}

impl Default for Goal {
    /// Creates a default `Goal` with no requirements.
    ///
    /// An empty goal represents a state where no specific conditions need to
    /// be satisfied. This is useful as a starting point for building goals
    /// using the builder pattern.
    ///
    /// # Example
    /// ```
    /// use rust_goap::prelude::*;
    ///
    /// let default_goal = Goal::default();
    /// assert!(default_goal.requirements.is_empty());
    ///
    /// // Can be used as a starting point for building
    /// let goal = Goal::default()
    ///     .with("condition", Assert::eq(true));
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

impl Goal {
    /// Creates a new empty goal.
    ///
    /// Returns a `Goal` with no requirements. Use the `with` method to add
    /// requirements using the builder pattern.
    ///
    /// # Returns
    /// A new `Goal` instance with no requirements.
    ///
    /// # Example
    /// ```
    /// use rust_goap::prelude::*;
    ///
    /// let empty_goal = Goal::new();
    /// assert!(empty_goal.requirements.is_empty());
    ///
    /// // Build a goal incrementally
    /// let goal = Goal::new()
    ///     .with("health", Assert::gt_eq(50))
    ///     .with("stamina", Assert::gt_eq(20));
    /// ```
    pub fn new() -> Self {
        Self { requirements: BTreeMap::new() }
    }

    /// Adds a requirement to the goal using the builder pattern.
    ///
    /// This method allows fluent chaining to build complex goals with multiple
    /// requirements. Each call adds a new requirement to the goal.
    ///
    /// # Arguments
    /// * `key` - The world state variable to check
    /// * `evaluate` - The assertion that must be true for that variable
    ///
    /// # Returns
    /// The modified `Goal` instance (for method chaining).
    ///
    /// # Example
    /// ```
    /// use rust_goap::prelude::*;
    ///
    /// let goal = Goal::new()
    ///     .with("health", Assert::gt_eq(80))      // Health >= 80
    ///     .with("ammo", Assert::gt(0))            // Ammo > 0
    ///     .with("has_key", Assert::eq(true));     // Has key == true
    ///
    /// assert_eq!(goal.requirements.len(), 3);
    ///
    /// // Verify the requirements were added correctly
    /// assert!(goal.requirements.contains_key("health"));
    /// assert!(goal.requirements.contains_key("ammo"));
    /// assert!(goal.requirements.contains_key("has_key"));
    /// ```
    pub fn with(mut self, key: impl Into<String>, evaluate: impl Into<Assert>) -> Self {
        self.requirements.insert(key.into(), evaluate.into());
        self
    }

    /// Creates a goal from a slice of requirements.
    ///
    /// This is a convenience constructor for creating goals from existing
    /// collections of requirements. It's particularly useful when you have
    /// requirements defined elsewhere in your code.
    ///
    /// # Arguments
    /// * `conditions` - A slice of (key, assertion) tuples representing the requirements
    ///
    /// # Returns
    /// A new `Goal` instance with all the specified requirements.
    ///
    /// # Example
    /// ```
    /// use rust_goap::prelude::*;
    ///
    /// // Define requirements separately
    /// let conditions = vec![
    ///     ("health".to_string(), Assert::gt_eq(75)),
    ///     ("stamina".to_string(), Assert::gt_eq(40)),
    ///     ("has_weapon".to_string(), Assert::eq(true)),
    /// ];
    ///
    /// // Create goal from the conditions
    /// let goal = Goal::from_reqs(&conditions);
    ///
    /// assert_eq!(goal.requirements.len(), 3);
    /// assert!(goal.requirements.contains_key("health"));
    /// assert!(goal.requirements.contains_key("stamina"));
    /// assert!(goal.requirements.contains_key("has_weapon"));
    /// ```
    pub fn from_reqs(conditions: &[(String, Assert)]) -> Self {
        let mut goal = Goal::new();
        for (k, v) in conditions {
            goal = goal.with(k, v.clone());
        }
        goal
    }

    /// Checks if a world state satisfies all requirements of this goal.
    ///
    /// This method evaluates whether the given world state meets all the
    /// conditions specified in the goal's requirements.
    ///
    /// # Arguments
    /// * `world_state` - The world state to check against
    ///
    /// # Returns
    /// `true` if all requirements are satisfied, `false` otherwise.
    ///
    /// # Example
    /// ```
    /// use rust_goap::prelude::*;
    ///
    /// // Create a goal
    /// let goal = Goal::new()
    ///     .with("health", Assert::gt_eq(50))
    ///     .with("has_key", Assert::eq(true));
    ///
    /// // Create a world state that satisfies the goal
    /// let good_state = WorldState::new()
    ///     .set("health", 75)
    ///     .set("has_key", true);
    ///
    /// // Create a world state that doesn't satisfy the goal
    /// let bad_state = WorldState::new()
    ///     .set("health", 30)    // Too low!
    ///     .set("has_key", true);
    ///
    /// // Note: The is_satisfied_by method doesn't exist in the current implementation,
    /// // but this example shows the conceptual usage. In practice, you would use
    /// // the planner to check if a state satisfies a goal.
    /// ```
    pub fn is_satisfied_by(&self, world_state: &crate::world_state::WorldState) -> bool {
        self.requirements.iter().all(|(key, assertion)| {
            world_state
                .0
                .get(key)
                .map(|value| crate::basic::assert::compare_values(assertion, value))
                .unwrap_or(false) // If key doesn't exist, requirement is not satisfied
        })
    }

    /// Returns the number of requirements in this goal.
    ///
    /// # Returns
    /// The count of requirements that must be satisfied for this goal.
    ///
    /// # Example
    /// ```
    /// use rust_goap::prelude::*;
    ///
    /// let goal = Goal::new()
    ///     .with("a", Assert::eq(1))
    ///     .with("b", Assert::eq(2))
    ///     .with("c", Assert::eq(3));
    ///
    /// assert_eq!(goal.requirement_count(), 3);
    /// ```
    pub fn requirement_count(&self) -> usize {
        self.requirements.len()
    }

    /// Checks if this goal has any requirements.
    ///
    /// # Returns
    /// `true` if the goal contains at least one requirement, `false` otherwise.
    ///
    /// # Example
    /// ```
    /// use rust_goap::prelude::*;
    ///
    /// let empty_goal = Goal::new();
    /// let populated_goal = Goal::new().with("key", Assert::eq(42));
    ///
    /// assert!(!empty_goal.has_requirements());
    /// assert!(populated_goal.has_requirements());
    /// ```
    pub fn has_requirements(&self) -> bool {
        !self.requirements.is_empty()
    }
}
