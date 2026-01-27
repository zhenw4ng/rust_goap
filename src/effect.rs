use crate::basic::mutation::Mutation;
use std::hash::{Hash, Hasher};

/// Represents the outcome of executing an action in a Goal-Oriented Action Planning (GOAP) system.
///
/// An `Effect` describes the changes that occur to the world state when an action is successfully
/// executed. It consists of one or more mutations that modify specific world state variables,
/// along with an associated cost that represents the "price" of applying these changes.
///
/// Effects are the mechanism by which actions transform the world, moving it closer to or
/// further from desired goals. The planner evaluates effects to determine the most efficient
/// sequence of actions to achieve a goal.
///
/// # Key Components
/// - **Mutations**: A list of changes to apply to the world state (set, delete, increment, decrement)
/// - **Cost**: A numerical value representing the "cost" of applying this effect (default: 1)
///
/// # Usage Example
/// ```
/// use rust_goap::prelude::*;
///
/// // Create an effect for an attack action
/// let attack_effect = Effect::new()
///     .with_mutation("enemy_health", Mutation::decrement("", 25))
///     .with_mutation("ammo_count", Mutation::decrement("", 1))
///     .with_cost(2); // Attacking has a higher cost than other actions
///
/// // Create an effect for a healing action
/// let heal_effect = Effect::new()
///     .with_mutation("player_health", Mutation::increment("", 50))
///     .with_mutation("medical_supplies", Mutation::decrement("", 1));
///     // Uses default cost of 1
///
/// // Create an effect for a resource gathering action
/// let gather_effect = Effect::new()
///     .with_mutation("wood_count", Mutation::increment("", 5))
///     .with_mutation("stamina", Mutation::decrement("", 10));
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Effect {
    /// The mutations to apply when this effect is executed.
    ///
    /// Each mutation modifies a specific world state variable. Multiple mutations
    /// can be combined in a single effect to represent complex state changes.
    ///
    /// # Example
    /// ```
    /// use rust_goap::prelude::*;
    ///
    /// let effect = Effect::new()
    ///     .with_mutation("health", Mutation::increment("", 20))
    ///     .with_mutation("hunger", Mutation::decrement("", 5));
    ///
    /// assert_eq!(effect.mutations.len(), 2);
    /// ```
    pub mutations: Vec<Mutation>,

    /// The cost associated with applying this effect.
    ///
    /// Cost represents the "price" of executing the action that produces this effect.
    /// Higher costs make actions less desirable to the planner. The default cost is 1.
    ///
    /// Costs are used by the planner to find the most efficient (lowest total cost)
    /// sequence of actions to achieve a goal.
    ///
    /// # Example
    /// ```
    /// use rust_goap::prelude::*;
    ///
    /// let cheap_effect = Effect::new(); // Default cost: 1
    /// let expensive_effect = Effect::new().with_cost(5);
    ///
    /// assert_eq!(cheap_effect.cost, 1);
    /// assert_eq!(expensive_effect.cost, 5);
    /// ```
    pub cost: usize,
}

impl Default for Effect {
    /// Creates a default `Effect` with no mutations and a cost of 1.
    ///
    /// This is useful as a starting point for building effects using the builder pattern.
    ///
    /// # Example
    /// ```
    /// use rust_goap::prelude::*;
    ///
    /// let default_effect = Effect::default();
    /// assert!(default_effect.mutations.is_empty());
    /// assert_eq!(default_effect.cost, 1);
    /// ```
    fn default() -> Self {
        Self {
            mutations: vec![],
            cost: 1,
        }
    }
}

impl Effect {
    /// Creates a new `Effect` with no mutations and default cost (1).
    ///
    /// This is the primary constructor for creating effects. Use the builder methods
    /// (`with_mutation`, `with_cost`) to customize the effect.
    ///
    /// # Returns
    /// A new `Effect` instance ready for customization.
    ///
    /// # Example
    /// ```
    /// use rust_goap::prelude::*;
    ///
    /// let effect = Effect::new();
    /// assert!(effect.mutations.is_empty());
    /// assert_eq!(effect.cost, 1);
    /// ```
    pub fn new() -> Self {
        Self {
            mutations: vec![],
            cost: 1,
        }
    }

    /// Adds a mutation to the effect using the builder pattern.
    ///
    /// This method allows fluent chaining to build complex effects with multiple
    /// state changes.
    ///
    /// # Arguments
    /// * `key` - The world state variable to modify
    /// * `mutation` - The mutation operation to apply to the variable
    ///
    /// # Returns
    /// The modified `Effect` instance (for method chaining).
    ///
    /// # Example
    /// ```
    /// use rust_goap::prelude::*;
    ///
    /// let effect = Effect::new()
    ///     .with_mutation("health", Mutation::increment("", 50))
    ///     .with_mutation("stamina", Mutation::decrement("", 20));
    ///
    /// assert_eq!(effect.mutations.len(), 2);
    /// ```
    pub fn with_mutation(mut self, key: impl Into<String>, mutation: Mutation) -> Self {
        // Convert the mutation to ensure it has the correct key
        let final_mutation = match mutation {
            Mutation::Set(_, value) => Mutation::Set(key.into(), value),
            Mutation::Delete(_) => Mutation::Delete(key.into()),
            Mutation::Increment(_, value) => Mutation::Increment(key.into(), value),
            Mutation::Decrement(_, value) => Mutation::Decrement(key.into(), value),
        };

        self.mutations.push(final_mutation);
        self
    }

    /// Sets the cost of applying this effect.
    ///
    /// Cost influences the planner's decision-making. Actions with lower cost
    /// effects are preferred when multiple action sequences can achieve the same goal.
    ///
    /// # Arguments
    /// * `cost` - The cost value (must be > 0 for meaningful planning)
    ///
    /// # Returns
    /// The modified `Effect` instance (for method chaining).
    ///
    /// # Example
    /// ```
    /// use rust_goap::prelude::*;
    ///
    /// let effect = Effect::new()
    ///     .with_mutation("distance", Mutation::decrement("", 10))
    ///     .with_cost(3); // Moving has a cost of 3
    ///
    /// assert_eq!(effect.cost, 3);
    /// ```
    pub fn with_cost(mut self, cost: usize) -> Self {
        self.cost = cost;
        self
    }

    /// Applies all mutations in this effect to a world state.
    ///
    /// This is a convenience method that applies each mutation in sequence,
    /// transforming the given world state according to the effect's specifications.
    ///
    /// # Arguments
    /// * `world_state` - The world state to modify
    ///
    /// # Example
    /// ```
    /// use rust_goap::prelude::*;
    ///
    /// let mut world_state = WorldState::new()
    ///     .set("health", 50)
    ///     .set("ammo", 10);
    ///
    /// let effect = Effect::new()
    ///     .with_mutation("health", Mutation::increment("", 25))
    ///     .with_mutation("ammo", Mutation::decrement("", 2));
    ///
    /// effect.apply_to(&mut world_state);
    /// // world_state now has health = 75 and ammo = 8
    /// ```
    pub fn apply_to(&self, world_state: &mut crate::world_state::WorldState) {
        use crate::basic::mutation::apply_mutator;

        for mutation in &self.mutations {
            apply_mutator(world_state, mutation);
        }
    }

    /// Returns the total number of mutations in this effect.
    ///
    /// # Returns
    /// The count of mutations that will be applied when this effect is executed.
    ///
    /// # Example
    /// ```
    /// use rust_goap::prelude::*;
    ///
    /// let effect = Effect::new()
    ///     .with_mutation("a", Mutation::set("", 1))
    ///     .with_mutation("b", Mutation::set("", 2))
    ///     .with_mutation("c", Mutation::set("", 3));
    ///
    /// assert_eq!(effect.mutation_count(), 3);
    /// ```
    pub fn mutation_count(&self) -> usize {
        self.mutations.len()
    }

    /// Checks if this effect has any mutations.
    ///
    /// # Returns
    /// `true` if the effect contains at least one mutation, `false` otherwise.
    ///
    /// # Example
    /// ```
    /// use rust_goap::prelude::*;
    ///
    /// let empty_effect = Effect::new();
    /// let populated_effect = Effect::new().with_mutation("key", Mutation::set("", 42));
    ///
    /// assert!(!empty_effect.has_mutations());
    /// assert!(populated_effect.has_mutations());
    /// ```
    pub fn has_mutations(&self) -> bool {
        !self.mutations.is_empty()
    }
}

impl Hash for Effect {
    /// Computes a hash value for the effect.
    ///
    /// The hash includes all mutations and the cost, ensuring that effects with
    /// different contents or costs hash to different values.
    ///
    /// This enables using `Effect` instances as keys in hash-based collections,
    /// which is useful for caching planning results or detecting duplicate effects.
    ///
    /// # Example
    /// ```
    /// use std::collections::hash_map::DefaultHasher;
    /// use std::hash::{Hash, Hasher};
    /// use rust_goap::prelude::*;
    ///
    /// let effect1 = Effect::new().with_mutation("health", Mutation::increment("", 10));
    /// let effect2 = Effect::new().with_mutation("health", Mutation::increment("", 10));
    /// let effect3 = Effect::new().with_mutation("health", Mutation::increment("", 20));
    ///
    /// let mut hasher1 = DefaultHasher::new();
    /// let mut hasher2 = DefaultHasher::new();
    /// let mut hasher3 = DefaultHasher::new();
    ///
    /// effect1.hash(&mut hasher1);
    /// effect2.hash(&mut hasher2);
    /// effect3.hash(&mut hasher3);
    ///
    /// assert_eq!(hasher1.finish(), hasher2.finish()); // Same content
    /// assert_ne!(hasher1.finish(), hasher3.finish()); // Different content
    /// ```
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.mutations.hash(state);
        self.cost.hash(state);
    }
}
