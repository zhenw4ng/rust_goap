use crate::basic::value::Value;
use crate::world_state::WorldState;

/// Represents a mutation operation that can be applied to a [`WorldState`].
///
/// `Mutation` describes how to modify the state of the world in the GOAP system.
/// Each mutation targets a specific state key and specifies an operation to
/// perform on its value.
///
/// # Examples
/// ```
/// use goap_lite::prelude::*;
///
/// // Create different types of mutations
/// let set_mutation = Mutation::set("health", 100);      // Set health to 100
/// let delete_mutation = Mutation::delete("temp_key");   // Remove a key
/// let inc_mutation = Mutation::increment("ammo", 10);   // Add 10 to ammo
/// let dec_mutation = Mutation::decrement("hunger", 5);  // Subtract 5 from hunger
/// ```
#[derive(Clone, Debug, PartialEq, Hash, Eq)]
pub enum Mutation {
    /// Set a value for a key, replacing any existing value
    Set(String, Value),
    /// Delete the target value by state key, removing it from the world state
    Delete(String),
    /// Increment a value for a key by a given amount
    Increment(String, Value),
    /// Decrement a value for a key by a given amount
    Decrement(String, Value),
}

impl Mutation {
    /// Creates a mutation that sets a key to a specific value.
    ///
    /// # Arguments
    /// * `key` - The state key to modify
    /// * `value` - The value to set
    ///
    /// # Examples
    /// ```
    /// use goap_lite::prelude::*;
    ///
    /// let mutation = Mutation::set("health", 100);
    /// assert!(matches!(mutation, Mutation::Set(key, Value::I64(100)) if key == "health"));
    /// ```
    pub fn set(key: impl Into<String>, value: impl Into<Value>) -> Self {
        Mutation::Set(key.into(), value.into())
    }

    /// Creates a mutation that deletes a key from the world state.
    ///
    /// # Arguments
    /// * `key` - The state key to delete
    ///
    /// # Examples
    /// ```
    /// use goap_lite::prelude::*;
    ///
    /// let mutation = Mutation::delete("temp_key");
    /// assert!(matches!(mutation, Mutation::Delete(key) if key == "temp_key"));
    /// ```
    pub fn delete(key: impl Into<String>) -> Self {
        Mutation::Delete(key.into())
    }

    /// Creates a mutation that increments a key's value by a specified amount.
    ///
    /// # Arguments
    /// * `key` - The state key to increment
    /// * `value` - The amount to add
    ///
    /// # Examples
    /// ```
    /// use goap_lite::prelude::*;
    ///
    /// let mutation = Mutation::increment("ammo", 10);
    /// assert!(matches!(mutation, Mutation::Increment(key, Value::I64(10)) if key == "ammo"));
    /// ```
    pub fn increment(key: impl Into<String>, value: impl Into<Value>) -> Self {
        Mutation::Increment(key.into(), value.into())
    }

    /// Creates a mutation that decrements a key's value by a specified amount.
    ///
    /// # Arguments
    /// * `key` - The state key to decrement
    /// * `value` - The amount to subtract
    ///
    /// # Examples
    /// ```
    /// use goap_lite::prelude::*;
    ///
    /// let mutation = Mutation::decrement("hunger", 5);
    /// assert!(matches!(mutation, Mutation::Decrement(key, Value::I64(5)) if key == "hunger"));
    /// ```
    pub fn decrement(key: impl Into<String>, value: impl Into<Value>) -> Self {
        Mutation::Decrement(key.into(), value.into())
    }
}

/// Applies a mutation to a world state.
///
/// This function modifies the given [`WorldState`] according to the specified
/// [`Mutation`]. It handles all four mutation types: Set, Delete, Increment,
/// and Decrement.
///
/// # Arguments
/// * `world_state` - The world state to modify
/// * `mutator` - The mutation to apply
///
/// # Examples
/// ```
/// use goap_lite::prelude::*;
///
/// let mut world_state = WorldState::new().set("health", 50);
/// let mutation = Mutation::increment("health", 25);
///
/// apply_mutator(&mut world_state, &mutation);
///
/// // Verify the mutation was applied by checking the internal state
/// // Note: WorldState doesn't have a public get() method, but we can
/// // verify through other means in real usage
/// ```
pub fn apply_mutator(world_state: &mut WorldState, mutator: &Mutation) {
    match mutator {
        Mutation::Set(key, value) => {
            world_state.0.insert(key.into(), *value);
        }
        Mutation::Delete(key) => {
            world_state.0.remove(key);
        }
        Mutation::Increment(key, value) => {
            if let Some(current_value) = world_state.0.get_mut(key) {
                *current_value += *value;
            }
        }
        Mutation::Decrement(key, value) => {
            if let Some(current_value) = world_state.0.get_mut(key) {
                *current_value -= *value;
            }
        }
    }
}

/// Formats a list of mutations for human-readable display.
///
/// This function creates a string representation of mutations, with each
/// mutation on its own line and a descriptive prefix indicating the type
/// of operation.
///
/// # Arguments
/// * `mutations` - A vector of mutations to format
///
/// # Returns
/// A formatted string with each mutation on a separate line.
///
/// # Examples
/// ```
/// use goap_lite::prelude::*;
///
/// let mutations = vec![
///     Mutation::set("health", 100),
///     Mutation::increment("ammo", 10),
///     Mutation::decrement("hunger", 5),
/// ];
///
/// let formatted = format_mutations(mutations);
/// assert!(formatted.contains("set: health = Value:I64(100)"));
/// assert!(formatted.contains("increment: ammo + Value:I64(10)"));
/// assert!(formatted.contains("decrement: hunger - Value:I64(5)"));
/// ```
pub fn format_mutations(mutations: Vec<Mutation>) -> String {
    let mut output = String::new();
    for mutation in mutations {
        match mutation {
            Mutation::Set(k, v) => output.push_str(&format!("set: {k} = {v}\n")),
            Mutation::Delete(k) => output.push_str(&format!("delete: {k}\n")),
            Mutation::Increment(k, v) => output.push_str(&format!("increment: {k} + {v}\n")),
            Mutation::Decrement(k, v) => output.push_str(&format!("decrement: {k} - {v}\n")),
        }
    }
    output
}
