mod data;
mod users;

// TODO: /login - login with discord
// TODO: /login?<TOKEN> - Show user info & signout button
// TODO: /wordgame - Store score in session cookie
// TODO: ADD Color Clock
// TODO: Implement games from https://gitlab.com/Fuzen-py/Games-rs

use crate::hosts::Hosts;
use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{web, HttpResponse, Result, Route};

use std::convert::Into;

lazy_static::lazy_static! {
    pub static ref TERA: ::tera::Tera = {
        let mut tera = ::tera::Tera::default();
        tera.add_raw_templates(vec![
            ("base", crate::statics::templates::BASE_TERA),
            ("content", crate::statics::templates::CONTENT_TERA),
            ("demos", crate::statics::templates::DEMOS_TERA),
            ("index", crate::statics::templates::INDEX_TERA),
            ("projects", crate::statics::templates::PROJECTS_TERA),
            ("login", crate::statics::templates::LOGIN_TERA),
            ("profile", crate::statics::templates::PROFILE_TERA),
        ])
        .expect("Failed to render templates");
        tera
    };
}

fn cafe() -> Route {
    web::get().guard(Hosts::FuzenCafe)
}

#[derive(Copy, Clone, Debug)]
pub struct ExternalResource {
    pub name: &'static str,
    pub url: &'static str,
}

pub enum FuzenCafeRoutes {
    Index,
    Demos,
    Projects,
    Static(Statics),
    Profile,
    WordGame,
}

impl FuzenCafeRoutes {
    pub fn ident_service() -> IdentityService<CookieIdentityPolicy> {
        let secret = std::env::var("SECRET_KEY").unwrap_or_else(|_| "0123".repeat(8));
        IdentityService::new(
            CookieIdentityPolicy::new(secret.as_bytes())
                .name("auth")
                .path("/")
                .domain(Hosts::FuzenCafe.as_ref())
                .max_age_time(chrono::Duration::days(1))
                .secure(true),
        )
    }
}

pub const EXTERNAL_DISCORD_TOKEN: ExternalResource = ExternalResource {
                    name: "discord_token",
                url: "https://discordapp.com/api/oauth2/token?grant_type=authorization_code&code={code}&redirect_uri={redirect_uri}"
};

pub const EXTERNAL_DISCORD_AUTHORIZE: ExternalResource = ExternalResource {
                name: "discord_authorize",
                url: "https://discordapp.com/api/oauth2/authorize?client_id={client_id}&redirect_uri={redirect_uri}&response_type=code&scope=identify"
};

pub enum Statics {
    WordGame(WordGames),
    Image(Images),
    Style,
    Favicon,
}

impl Into<Route> for Statics {
    fn into(self) -> Route {
        match self {
            Statics::Image(inner) => inner.into(),
            Statics::WordGame(inner) => inner.into(),
            Statics::Style => cafe().to_async(css),
            Statics::Favicon => cafe().to_async(favicon),
        }
    }
}

pub enum Images {
    FuzenInfo,
}

impl Into<Route> for Images {
    fn into(self) -> Route {
        match self {
            Images::FuzenInfo => cafe().to_async(img),
        }
    }
}

pub enum WordGames {
    MainJS,
    MainCSS,
}

impl Into<Route> for WordGames {
    fn into(self) -> Route {
        match self {
            WordGames::MainCSS => cafe().to_async(wordgame_css),
            WordGames::MainJS => cafe().to_async(wordgame_js),
        }
    }
}

impl Into<Route> for FuzenCafeRoutes {
    fn into(self) -> Route {
        match self {
            FuzenCafeRoutes::Demos => cafe().to_async(demos),
            FuzenCafeRoutes::Index => cafe().to_async(index),
            FuzenCafeRoutes::Profile => cafe().to_async(users::profile),
            FuzenCafeRoutes::Projects => cafe().to_async(projects),
            FuzenCafeRoutes::Static(inner) => inner.into(),
            FuzenCafeRoutes::WordGame => cafe().to_async(wordgame_main),
        }
    }
}

fn index() -> Result<HttpResponse> {
    let mut context = ::tera::Context::default();
    context.insert("blog", &data::DEMO_BLOGS.to_vec());
    TERA.render("index", &context)
        .and_then(|result| Ok(HttpResponse::Ok().content_type("text/html").body(result)))
        .map_err(|_| actix_web::error::ErrorNotFound("Render failed"))
}

fn wordgame_main() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(crate::statics::wordgame::INDEX_HTML)
}

fn wordgame_css() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/css")
        .body(crate::statics::wordgame::MAIN_CSS)
}

fn wordgame_js() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/javascript")
        .body(crate::statics::wordgame::MAIN_JS)
}

fn demos() -> Result<HttpResponse> {
    let mut context = ::tera::Context::default();
    context.insert(
        "demos",
        &vec![data::Demo {
            name: String::from("Color Clock"),
            link: String::from("https://fuzen-py.github.io/color-clock/"),
            description: String::from("Changes color with time"),
            src: Some(String::from("https://github.com/Fuzen-py/color-clock")),
        }],
    );
    TERA.render("demos", &context)
        .and_then(|result| Ok(HttpResponse::Ok().content_type("text/html").body(result)))
        .map_err(|_| actix_web::error::ErrorNotFound("Render failed"))
}

fn projects() -> Result<HttpResponse> {
    let mut context = ::tera::Context::default();
    context.insert("projects", &[(); 1]);
    TERA.render("projects", &context)
        .and_then(|result| Ok(HttpResponse::Ok().content_type("text/html").body(result)))
        .map_err(|_| actix_web::error::ErrorNotFound("Render failed"))
}

fn favicon() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("image/png")
        .body(crate::statics::images::FUZEN_CAFE_FAVICON)
}

fn css() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/css")
        .body(crate::statics::STYLE_SHEET)
}

fn img() -> HttpResponse {
    HttpResponse::Ok()
        .content_type("image/png")
        .body(crate::statics::images::FUZEN_INFO_FAVICON)
}
