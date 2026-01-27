use rust_goap::prelude::*;

fn main() {
    // Define the initial world state
    let start = WorldState::new()
        .set("is_hungry", true)
        .set("has_food", false);

    // Define the goal: not hungry
    let goal = Goal::new().with("is_hungry", Assert::eq(false));

    // Define available actions
    let buy_food = Action::new("buy_food").with_effect(Effect {
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
    if let Some(plan) = make_plan(&start, &actions, &goal) {
        println!("{}", format_plan(plan.clone()));
    }
}
