# GOAP Lite

[![Crates.io](https://img.shields.io/crates/v/goap_lite)](https://crates.io/crates/goap_lite)
[![Documentation](https://docs.rs/goap_lite/badge.svg)](https://docs.rs/goap_lite)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A lightweight, efficient Goal-Oriented Action Planning (GOAP) library for Rust, designed for game AI and decision-making systems. Extracted from the core GOAP concepts of the dogoap project with enhanced features and flexibility.

## Features

- **Simple API**: Easy-to-use interface for defining actions, goals, and world states
- **Efficient Planning**: Uses A* pathfinding algorithm with custom heuristics
- **Flexible State System**: Supports various value types (bool, i64, f64, String)
- **Cost-Based Optimization**: Finds the lowest-cost path to achieve goals
- **Human-Readable Output**: Built-in plan formatting for debugging and visualization
- **Minimal Dependencies**: Only depends on the `pathfinding` crate
- **Future JSON Support**: Planned support for JSON serialization to create actions dynamically

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
goap_lite = "0.1"
```

## Quick Start

```rust
use goap_lite::prelude::*;

fn main() {
    // Define the initial world state
    let start = WorldState::new()
        .set("is_hungry", true)
        .set("has_food", false);

    // Define the goal: not hungry
    let goal = Goal::new().with("is_hungry", Assert::eq(false));

    // Define available actions
    let buy_food = Action::new("buy_food")
        .with_effect(Effect {
            mutations: vec![Mutation::set("has_food", true)],
            cost: 2,
        });

    let eat = Action::new("eat")
        .with_precondition(("has_food", Assert::eq(true)))
        .with_effect(Effect {
            mutations: vec![
                Mutation::set("is_hungry", false),
                Mutation::set("has_food", false),
            ],
            cost: 1,
        });

    let actions = vec![buy_food, eat];

    // Find the optimal plan
    if let Some((plan, cost)) = make_plan(&start, &actions, &goal) {
        println!("Found plan with cost: {}", cost);
        println!("{}", format_plan(plan));
    }
}
```

## Core Concepts

### World State
The current state of the world, represented as a key-value map:

```rust
let state = WorldState::new()
    .set("health", 100_i64)
    .set("has_weapon", true)
    .set("enemy_nearby", false);
```

### Goals
Desired states to achieve, with assertions on values:

```rust
let goal = Goal::new()
    .with("health", Assert::gt_eq(50_i64))
    .with("enemy_nearby", Assert::eq(false));
```

### Actions
Actions that can be performed, with preconditions and effects:

```rust
let heal = Action::new("heal")
    .with_precondition(("has_medicine", Assert::eq(true)))
    .with_effect(Effect {
        mutations: vec![Mutation::increment("health", 20)],
        cost: 3,
    });
```

### Effects
Changes to the world state when an action is performed:

```rust
Effect {
    mutations: vec![
        Mutation::set("has_medicine", false),
        Mutation::increment("health", 20),
    ],
    cost: 3,
}
```

## Mutation Types

- `Mutation::set(key, value)` - Set a value
- `Mutation::increment(key, amount)` - Increment a numeric value
- `Mutation::decrement(key, amount)` - Decrement a numeric value
- `Mutation::delete(key)` - Remove a key from the state

## Assertion Types

- `Assert::eq(value)` - Equal to
- `Assert::not_eq(value)` - Not equal to
- `Assert::gt(value)` - Greater than
- `Assert::gt_eq(value)` - Greater than or equal to
- `Assert::lt(value)` - Less than
- `Assert::lt_eq(value)` - Less than or equal to

## Examples

The repository includes several examples:

### Basic Example
```bash
cargo run --example basic
```

### Complex Planning Example
```bash
cargo run --example long_plan
```

## API Documentation

### Main Functions

- `make_plan(start, actions, goal)` - Find optimal plan from start to goal
- `make_plan_with_strategy(strategy, start, actions, goal)` - Plan with specific strategy
- `get_effects_from_plan(plan)` - Extract actions and effects from a plan
- `format_plan(plan)` - Format plan as human-readable string

### Core Types

- `WorldState` - Represents the current state of the world
- `Goal` - Desired state with requirements
- `Action` - Action that can be performed
- `Effect` - Changes caused by an action
- `Node` - Internal planning node (graph search)
- `PlanningStrategy` - Planning algorithm strategy

## Performance

The library uses the A* algorithm with the following optimizations:
- Custom heuristic based on goal distance
- Efficient state comparison and cloning
- Early pruning of invalid action sequences

## Roadmap

### Planned Features
- **JSON Serialization**: Create actions, goals, and world states from JSON configuration
- **YAML Support**: Alternative configuration format for easier editing
- **Plugin System**: Extensible action and effect system
- **Parallel Planning**: Multi-threaded plan search for complex scenarios
- **Visualization Tools**: Graph visualization of planning process

### JSON Example (Future)
```json
{
  "actions": [
    {
      "key": "eat",
      "preconditions": [
        {
          "key": "has_food",
          "assertion": {
            "type": "eq",
            "value": true
          }
        }
      ],
      "effect": {
        "mutations": [
          {
            "type": "set",
            "key": "is_hungry",
            "value": false
          }
        ],
        "cost": 1
      }
    }
  ]
}
```

## Use Cases

- **Game AI**: NPC behavior planning, enemy AI, companion AI
- **Robotics**: Task planning, behavior trees
- **Simulations**: Agent-based modeling, decision systems
- **Automation**: Workflow planning, process optimization
- **Education**: Teaching AI planning concepts

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m "Add some amazing feature"`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

This project was inspired by and extracts core GOAP concepts from the [dogoap](https://github.com/victorb/dogoap) repository, a Rust implementation of Goal-Oriented Action Planning. GOAP Lite builds upon these foundations with a focus on simplicity, performance, and future extensibility including JSON serialization support.

---

**GOAP Lite** - Simple, efficient, and extensible goal-oriented planning for Rust applications.
