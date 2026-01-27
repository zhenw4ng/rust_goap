use rust_goap::prelude::*;

fn main() {
    let start = WorldState::new().set("is_hungry", true);

    let goal = Goal::new().with("is_hungry", Assert::eq(false));

    let eat_action = Action {
        key: "eat".to_string(),
        preconditions: vec![],
        effect: Some(Effect {
            mutations: vec![Mutation::set("is_hungry", false)],
            cost: 1,
        }),
    };

    let actions: Vec<Action> = vec![eat_action];
    let plan = make_plan(&start, &actions[..], &goal);
    println!("{}", format_plan(plan.unwrap()));
}
