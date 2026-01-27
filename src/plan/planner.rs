//! Planning module for GOAP (Goal-Oriented Action Planning) system.
//!
//! This module provides the core planning algorithms that find optimal sequences
//! of actions to transition from an initial world state to a desired goal state.
//! It uses the A* pathfinding algorithm with custom heuristics and state
//! transition logic.

use crate::plan::node::Node;
use crate::{
    action::Action,
    basic::assert::compare_values,
    basic::mutation::{apply_mutator, format_mutations},
    effect::Effect,
    goal::Goal,
    world_state::WorldState,
};

/// Heuristic function for A* pathfinding.
///
/// Estimates the cost to reach the goal from the given node.
/// Uses the `distance_to_goal` method of the world state to calculate
/// how far the current state is from satisfying all goal requirements.
///
/// # Arguments
/// * `node` - Current node in the search graph
/// * `goal` - Target goal state
///
/// # Returns
/// Estimated cost (as usize) to reach the goal from this node
fn heuristic(node: &Node, goal: &Goal) -> usize {
    node.state().distance_to_goal(goal) as usize
}

/// Generates successor nodes for the A* pathfinding algorithm.
///
/// For a given node, returns all possible next nodes by applying
/// valid actions from the available action list. Each successor
/// includes the cost of applying the action's effect.
///
/// # Arguments
/// * `node` - Current node to expand
/// * `actions` - List of available actions
///
/// # Returns
/// Iterator over (successor_node, transition_cost) pairs
fn successors<'a>(
    node: &'a Node,
    actions: &'a [Action],
) -> impl Iterator<Item = (Node, usize)> + 'a {
    let state = node.state();
    actions.iter().filter_map(move |action| {
        // Skip actions whose preconditions aren't met or have no effect
        if !action.check_preconditions(state) || action.effect.is_none() {
            return None;
        }

        let effect = action.effect.as_ref().unwrap();

        // Apply the effect's mutations to create the new state
        let mut new_state = state.clone();
        for mutator in &effect.mutations {
            apply_mutator(&mut new_state, mutator);
        }

        // Create a new effect with the same properties
        let new_effect = Effect { mutations: effect.mutations.clone(), cost: effect.cost };

        // Return the successor node with its transition cost
        Some((Node::Effect((action.key.clone(), new_effect, new_state)), effect.cost))
    })
}

/// Checks if a node satisfies all goal requirements.
///
/// Compares the world state in the node against all requirements
/// specified in the goal. Returns true only if all requirements
/// are satisfied according to their assertion rules.
///
/// # Arguments
/// * `node` - Node to check
/// * `goal` - Goal containing requirements to satisfy
///
/// # Returns
/// `true` if the node's state satisfies all goal requirements, `false` otherwise
fn is_goal(node: &Node, goal: &Goal) -> bool {
    goal.requirements.iter().all(|(key, required_value)| {
        let state_value = match node.state().0.get(key) {
            Some(val) => val,
            None => {
                // If a goal requirement key is not in the state,
                // the goal cannot be satisfied
                return false;
            },
        };
        compare_values(required_value, state_value)
    })
}

/// Planning strategies for finding paths from start to goal.
///
/// Different strategies can be used depending on the planning requirements,
/// though currently only `StartToGoal` is implemented.
#[derive(Default, Copy, Clone, Debug)]
pub enum PlanningStrategy {
    #[default]
    /// Starts from the initial state and searches forward to find the
    /// optimal path to the goal state.
    ///
    /// This strategy evaluates all possible action sequences from the
    /// starting point, ensuring the lowest-cost path is found, though
    /// it may take longer than alternative approaches.
    StartToGoal,
}

/// Creates a plan using a specified planning strategy.
///
/// This is the lower-level planning function that allows specifying
/// which planning strategy to use. For most use cases, prefer [`make_plan`].
///
/// # Arguments
/// * `strategy` - Planning strategy to use
/// * `start` - Initial world state
/// * `actions` - Available actions that can be performed
/// * `goal` - Desired goal state
///
/// # Returns
/// * `Some((path, total_cost))` if a plan is found, where `path` is a sequence
///   of nodes from start to goal and `total_cost` is the sum of all action costs
/// * `None` if no valid plan exists
///
/// # See Also
/// [`make_plan`] - Higher-level function that uses the default strategy
pub fn make_plan_with_strategy(
    strategy: PlanningStrategy,
    start: &WorldState,
    actions: &[Action],
    goal: &Goal,
) -> Option<(Vec<Node>, usize)> {
    match strategy {
        PlanningStrategy::StartToGoal => {
            let start_node = Node::State(start.clone());
            pathfinding::directed::astar::astar(
                &start_node,
                |node| successors(node, actions).collect::<Vec<_>>().into_iter(),
                |node| heuristic(node, goal),
                |node| is_goal(node, goal),
            )
        },
    }
}

/// Creates an optimal plan from start state to goal state.
///
/// This is the main planning function that finds the lowest-cost sequence
/// of actions to transition from the initial world state to a state that
/// satisfies all goal requirements.
///
/// Uses the A* pathfinding algorithm with custom heuristics to efficiently
/// search through the space of possible action sequences.
///
/// # Arguments
/// * `start` - Initial world state
/// * `actions` - Available actions that can be performed
/// * `goal` - Desired goal state with requirements
///
/// # Returns
/// * `Some((path, total_cost))` if a plan is found
/// * `None` if no valid plan exists
///
/// # Example
/// ```rust
/// use rust_goap::prelude::*;
///
/// let start = WorldState::new().set("has_food", false).set("is_hungry", true);
/// let goal = Goal::new().with("is_hungry", Assert::eq(false));
/// let eat_action = Action {
///     key: "eat".to_string(),
///     preconditions: vec![("has_food".to_string(), Assert::eq(true))],
///     effect: Some(Effect {
///         mutations: vec![Mutation::set("is_hungry", false)],
///         cost: 1,
///     }),
/// };
///
/// if let Some((plan, cost)) = make_plan(&start, &[eat_action], &goal) {
///     println!("Found plan with cost: {}", cost);
/// }
/// ```
pub fn make_plan(
    start: &WorldState,
    actions: &[Action],
    goal: &Goal,
) -> Option<(Vec<Node>, usize)> {
    // Default to using Start -> Goal planning
    make_plan_with_strategy(PlanningStrategy::StartToGoal, start, actions, goal)
}

/// Extracts all effects from a plan, filtering out initial state nodes.
///
/// Converts a plan (sequence of nodes) into an iterator over the actual
/// actions and their effects. This is useful for executing the plan
/// or analyzing the specific actions that need to be performed.
///
/// # Arguments
/// * `plan` - Plan containing both state and effect nodes
///
/// # Returns
/// Iterator over tuples of (action_key, effect, resulting_state)
///
/// # Note
/// Initial state nodes (Node::State) are filtered out since they
/// don't represent actions that need to be executed.
pub fn get_effects_from_plan(
    plan: impl IntoIterator<Item = Node>,
) -> impl Iterator<Item = (String, Effect, WorldState)> {
    plan.into_iter().filter_map(|node| match node {
        Node::Effect((action_key, effect, state)) => Some((action_key, effect, state)),
        Node::State(_) => None,
    })
}

/// Formats a plan into a human-readable string for debugging or display.
///
/// Creates a detailed textual representation of a plan showing:
/// - Initial world state
/// - Each action to execute with its mutations
/// - Intermediate states after each action
/// - Final state and total plan cost
///
/// # Arguments
/// * `plan` - Tuple containing the node sequence and total cost
///
/// # Returns
/// Formatted string representation of the plan
///
/// # Example Output
/// ```text
///         = INITIAL STATE
///         is_hungry = Value:Bool(true)
///
///         ---
///         = DO ACTION "eat"
///         MUTATES:
///         set: is_hungry = Value:Bool(false)
///         current state:
///         WorldState({"is_hungry": Bool(false)})
///
///         ---
///         = FINAL STATE (COST: 1)
///         is_hungry = Value:Bool(false)
/// ```
#[must_use]
pub fn format_plan(plan: (Vec<Node>, usize)) -> String {
    let mut output = String::new();
    let nodes = plan.0;
    let cost = plan.1;
    let mut last_state: WorldState = WorldState::new();

    for node in nodes {
        match node {
            Node::Effect((action_key, effect, state)) => {
                output.push_str(&format!("\t\t= DO ACTION {:#?}\n", action_key));
                output.push_str("\t\tMUTATES:\n");
                output.push_str(&format_mutations(effect.mutations.clone()));
                output.push_str(&format!("current state:\n{:?}\n", state));
                last_state = state.clone();
            },
            Node::State(s) => {
                output.push_str("\t\t= INITIAL STATE\n");
                for (k, v) in &s.0 {
                    output.push_str(&format!("\t\t{k} = {v}\n"));
                }
                last_state = s.clone();
            },
        }
        output.push_str("\n\t\t---\n");
    }

    output.push_str(&format!("\t\t= FINAL STATE (COST: {cost})\n"));
    for (k, v) in &last_state.0 {
        output.push_str(&format!("\t\t{k} = {v}\n"));
    }

    output
}
