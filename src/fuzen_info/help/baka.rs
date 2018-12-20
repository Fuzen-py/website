use super::Help;

pub fn help() -> Help {
    Help {
        description: "Call someone a baka",
        base_route: "/baka",
        base_route_function: None,
        base_route_arguments: None,
        routes: vec![(
            "/baka/<name>",
            ("Calls <name> a baka", Vec::with_capacity(0)),
        )],
        examples: vec![("/baka/onii-chan", "onii-chan's a baka!😤")],
    }
}
