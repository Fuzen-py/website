use super::Help;

pub fn help() -> Help {
    Help {
        description: "Get a random word",
        base_route: "/randomword",
        base_route_function: Some("Get a random word"),
        base_route_arguments: Some(vec![
            [
                "min=<num=0>",
                "Restrict the random word to be at least <num> in length.",
            ],
            [
                "max=<num=0>",
                "Restrict the random word to be no more than <num> in length.",
            ],
            [
                "length=<num=0>",
                "Restrict the random word to be exactly <num> in length.",
            ],
            [
                "count=<num=1>",
                " Returns as many random words as conditions allow, but no more than count.",
            ],
        ]),
        routes: vec![],
        examples: vec![
            ("/randomword?min=10", "catholicized"),
            ("/randomword?max=10", "cabrie"),
            ("/randomword?count=1", "epulosis"),
            (
                "/randomword?min=25&max=30&count=1",
                "hydroxydesoxycorticosterone",
            ),
        ],
    }
}
