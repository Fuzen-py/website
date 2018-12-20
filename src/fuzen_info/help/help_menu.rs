use super::Help;

pub fn help() -> Help {
    Help {
        description: "Help",
        base_route: "/help",
        base_route_function: None,
        base_route_arguments: None,
        routes: vec![
            ("/help/baka", ("Help menu for baka", Vec::with_capacity(0))),
            (
                "/help/fractal.png",
                ("Help menu for fractal", Vec::with_capacity(0)),
            ),
            (
                "/help/hello",
                ("Help menu for hello", Vec::with_capacity(0)),
            ),
            ("/help/help", ("Help menu for help", Vec::with_capacity(0))),
            (
                "/help/randomword",
                ("Helo menu for randomword", Vec::with_capacity(0)),
            ),
            ("/help/totp", ("Helo menu for totp", Vec::with_capacity(0))),
        ],
        examples: vec![("/help/help", "Displays this page")],
    }
}
