//! This example shows how a plan with lots of steps can be created.

use rust_goap::prelude::*;

fn main() {
    let start = WorldState::new()
        .set("energy", 30_i64)
        .set("hunger", 70_i64)
        .set("gold", 0_i64);

    let goal = Goal::new().with("gold", Assert::eq(7_i64));

    let sleep_action = Action::new("sleep").with_effect(Effect {
        mutations: vec![Mutation::increment("energy", 10)],
        cost: 1,
    });

    let eat_action = Action::new("eat")
        .with_effect(Effect {
            mutations: vec![Mutation::decrement("hunger", 10)],
            cost: 1,
        })
        .with_precondition(("energy", Assert::gt_eq(26_i64)));

    let rob_people = Action::new("rob")
        .with_effect(Effect {
            mutations: vec![
                Mutation::increment("gold", 1),
                Mutation::decrement("energy", 5),
                Mutation::increment("hunger", 5),
            ],
            cost: 1,
        })
        .with_precondition(("hunger", Assert::lt_eq(50_i64)))
        .with_precondition(("energy", Assert::gt_eq(50_i64)));

    let actions: Vec<Action> = vec![sleep_action, eat_action, rob_people];

    let plan = make_plan(&start, &actions[..], &goal);

    println!("{}", format_plan(plan.clone().unwrap()));
}
