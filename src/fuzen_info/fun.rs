use actix_web::{Path, Result};

#[derive(Deserialize)]
pub struct Info {
    name: String,
}
#[derive(Serialize)]
pub struct Hello {
    pub name: String,
}

impl std::fmt::Display for Hello {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Hello, {}!👋", self.name)
    }
}

impl Hello {
    //Rust > 1.31 #[allow(clippy::needless_pass_by_value)]
    pub fn route(info: Path<Info>) -> Result<String> {
        Ok(Hello {
            name: info.into_inner().name,
        }
        .to_string())
    }
}

#[derive(Serialize)]
pub struct Baka {
    name: String,
}

impl std::fmt::Display for Baka {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}'s a baka!😤", self.name)
    }
}
impl Baka {
    //Rust > 1.31 #[allow(clippy::needless_pass_by_value)]
    pub fn route(info: Path<Info>) -> Result<String> {
        Ok(Baka {
            name: info.into_inner().name,
        }
        .to_string())
    }
}
