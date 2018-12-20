use super::Help;

pub fn help() -> Help {
    Help {
        description: "Greet someone",
        base_route: "/hello",
        base_route_function: None,
        base_route_arguments: None,
        routes: vec![(
            "/hello/<name>",
            ("Say hello to <name>", Vec::with_capacity(0)),
        )],
        examples: vec![("/hello/world", "Hello, world! 👋")],
    }
}
