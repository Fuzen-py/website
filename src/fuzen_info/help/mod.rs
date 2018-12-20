#![deny(unused)]
use actix_web::{Path, Result};

mod baka;
mod fractal;
mod hello;
mod help_menu;
mod randomword;
mod totp;

/// Help Structure
#[derive(Serialize)]
pub struct Help {
    /// Description of the entry
    pub description: &'static str,
    /// Base Route of the entry
    pub base_route: &'static str,
    /// Function of the base route if any
    pub base_route_function: Option<&'static str>,
    /// Arguments of the base route if any and their functionality
    pub base_route_arguments: Option<Vec<[&'static str; 2]>>,
    /// route, function of route, optional arguments and their functionality  
    pub routes: Vec<(&'static str, (&'static str, Vec<[&'static str; 2]>))>,
    /// Examples, function of example, example output (plaintext)
    pub examples: Vec<(&'static str, &'static str)>,
}

impl std::fmt::Display for Help {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut msg = String::from(self.description);
        msg.push('\n');
        msg.push_str(self.base_route);
        if let Some(function) = self.base_route_function {
            msg.push_str(" - ");
            msg.push_str(function);
        }
        msg.push('\n');
        if let Some(ref args) = self.base_route_arguments {
            for arg in args {
                msg.push_str(&format!("?{} - {}\n", arg[0], arg[1]))
            }
        }
        if !self.routes.is_empty() {
            msg.push_str("Routes:\n");
            for route in &self.routes {
                let route_info = &route.1;
                msg.push_str(&format!("{} - {}", route.0, route_info.0));
                for arg in &route_info.1 {
                    msg.push_str(&format!("\n?{} - {}", arg[0], arg[1]))
                }
                msg.push('\n')
            }
        }
        if !self.examples.is_empty() {
            msg.push_str("Examples:\n");
            for example in &self.examples {
                msg.push_str(&format!("{} - {}\n", example.0, example.1))
            }
        }
        write!(f, "{}", msg)
    }
}

#[derive(Deserialize)]
pub struct Info {
    #[serde(default)]
    route: String,
}

pub fn help(info: Path<Info>) -> Result<String> {
    let route = info.route.to_ascii_lowercase();
    Ok(match route.as_str() {
        "baka" => baka::help(),
        "fractal.png" => fractal::help(),
        "hello" => hello::help(),
        "randomword" => randomword::help(),
        "totp" => totp::help(),
        _ => help_menu::help(),
    }
    .to_string())
}
