#![deny(unused)]
use actix_web::{Path, Result};

static BAKA_HELP: &'static str = include_str!("baka.help");
static FRACTAL_HELP: &'static str = include_str!("fractal.help");
static HELLO_HELP: &'static str = include_str!("hello.help");
static HELP_HELP: &'static str = include_str!("help.help");
static RANDOMWORD_HELP: &'static str = include_str!("randomword.help");
static TOTP_HELP: &'static str = include_str!("totp.help");

#[derive(Deserialize)]
pub struct Info {
    #[serde(default)]
    route: String,
}

pub fn help(info: Path<Info>) -> Result<&'static str> {
    let route = info.route.to_ascii_lowercase();
    Ok(match route.as_str() {
        "baka" => BAKA_HELP,
        "fractal.png" => FRACTAL_HELP,
        "hello" => HELLO_HELP,
        "randomword" => RANDOMWORD_HELP,
        "totp" => TOTP_HELP,
        _ => HELP_HELP,
    })
}
