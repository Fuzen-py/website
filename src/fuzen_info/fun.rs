use actix_web::{Path, Result};

#[derive(Deserialize)]
pub struct Info {
    #[serde(default)]
    name: String,
}

//Rust > 1.31 #[allow(clippy::needless_pass_by_value)]
pub fn hello(info: Path<Info>) -> Result<String> {
    if info.name.is_empty() {
        Ok(String::from(
            "Usage: /hello/<name>\nExample: /hello/world - Hello, World! 👋",
        ))
    } else {
        Ok(format!("Hello, {}! 👋", info.name))
    }
}

//Rust > 1.31 #[allow(clippy::needless_pass_by_value)]
pub fn baka(info: Path<Info>) -> Result<String> {
    if info.name.is_empty() {
        Ok(String::from(
            "Usage: /baka/<name>\nExample: /baka/Onii-chan - Onii-chan's a baka! 😤",
        ))
    } else {
        Ok(format!("{}'s a baka!😤", info.name))
    }
}
