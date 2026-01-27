/// Represents a node in the planning graph for pathfinding algorithms.
///
/// A node can be either:
/// - The initial world state (starting point)
/// - A world state after applying an action's effect
///
/// This enum is used by the A* pathfinding algorithm to explore possible
/// state transitions and find optimal paths from start to goal.
use crate::effect::Effect;
use crate::world_state::WorldState;

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum Node {
    /// The initial world state at the start of planning.
    State(WorldState),
    /// Represents a transition after applying an action's effect.
    /// Contains: (action_key, effect_applied, resulting_world_state)
    Effect((String, Effect, WorldState)),
}

impl Node {
    /// Returns a reference to the world state contained in this node.
    ///
    /// For `State` variants, returns the initial world state.
    /// For `Effect` variants, returns the world state after applying the effect.
    pub fn state(&self) -> &WorldState {
        match self {
            Node::State(state) => state,
            Node::Effect((_, _, state)) => state,
        }
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Node::Effect(effect) => effect.fmt(f),
            Node::State(state) => state.fmt(f),
        }
    }
}
