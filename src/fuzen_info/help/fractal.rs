use super::Help;

pub fn help() -> Help {
    Help {
        description: "Generate a fractal PNG",
        base_route: "/fractal.png",
        base_route_function: Some("Generate a fractal PNG"),
        base_route_arguments: None,
        routes: vec![(
            "/fractal.png",
            ("Generate a fractal", Vec::with_capacity(0)),
        )],
        examples: vec![("/fractal.png", "Returns a 700x700 fractal")],
    }
}
