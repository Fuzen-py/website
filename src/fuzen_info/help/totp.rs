use super::Help;

pub fn help() -> Help {
    Help {
        description: "Generate a time-based one time password",
        base_route: "/totp",
        base_route_function: Some("Generate a time-based one time password"),
        base_route_arguments: Some(vec![
            [
                "token=<key>",
                "Required: Takes a base32 key to gen the password",
            ],
            ["period=<int:30>", "Password period, defaults to 30"],
            [
                "intial_time=<int:0>",
                "Change the inital time, defaults to 0",
            ],
            [
                "base=<list[u8]: base10>",
                "Change the base used for generating, defaults to base10",
            ],
            [
                "length=<int:6>",
                "Change the length of the key, max length depends on the base, defaults to 6",
            ],
        ]),
        routes: Vec::with_capacity(0),
        examples: vec![(
            "/totp?token=n3zvdxltyjnunmwyt6gy5mjxkizbbaxl&length=7",
            "1218391",
        )],
    }
}

// n3zv dxlt yjnu nmwy t6gy 5mjx kizb baxl
