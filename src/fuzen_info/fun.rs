use actix_web::{web, Result};

#[derive(serde::Deserialize)]
pub struct Info {
    name: String,
}
#[derive(serde::Serialize)]
pub struct Hello {
    pub name: String,
}

impl std::fmt::Display for Hello {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "Hello, {}!👋", self.name)
    }
}

impl Hello {
    pub fn route(info: web::Path<Info>) -> Result<String> {
        Ok(Hello {
            name: info.name.clone(),
        }
        .to_string())
    }
}

#[derive(serde::Serialize)]
pub struct Baka {
    name: String,
}

impl std::fmt::Display for Baka {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}'s a baka!😤", self.name)
    }
}
impl Baka {
    pub fn route(info: web::Path<Info>) -> Result<String> {
        Ok(Baka {
            name: info.name.clone(),
        }
        .to_string())
    }
}
