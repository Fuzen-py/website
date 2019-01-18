mod data;
mod users;

// TODO: /login - login with discord
// TODO: /login?<TOKEN> - Show user info & signout button
// TODO: /wordgame - Store score in session cookie
// TODO: ADD Color Clock
// TODO: Implement games from https://gitlab.com/Fuzen-py/Games-rs

use actix_web::{
    middleware::session::{CookieSessionBackend, SessionStorage},
    HttpRequest, HttpResponse, Result,
};

lazy_static! {
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

pub fn route() -> ::actix_web::App {
    crate::hosts::Hosts::FuzenCafe
        .filter(::actix_web::App::new())
        .resource("/", |r| r.f(index))
        .resource("/demos", |r| r.f(demos))
        .resource("/projects", |r| r.f(projects))
        .resource("/favicon.ico", |r| r.f(favicon))
        .resource("/static/style.css", |r| r.f(css))
        .resource("/static/images/FuzenInfo.png", |r| r.f(img))
        .resource("/wordgame", |r| r.f(wordgame_main))
        .resource("/static/wordgame/main.css", |r| r.f(wordgame_css))
        .resource("/static/wordgame/main.js", |r| r.f(wordgame_js))
        .resource("/profile", |r| r.f(users::profile))
        .external_resource("discord_authorize", "https://discordapp.com/api/oauth2/authorize?client_id={client_id}&redirect_uri={redirect_uri}&response_type=code&scope=identify")
        .external_resource("discord_token", "https://discordapp.com/api/oauth2/token?grant_type=authorization_code&code={code}&redirect_uri={redirect_uri}")
        .middleware(SessionStorage::new(
            CookieSessionBackend::private(&[0; 32]).secure(true),
        ))
}

fn index(_: &HttpRequest) -> Result<HttpResponse> {
    let mut context = ::tera::Context::default();
    context.insert("blog", &data::DEMO_BLOGS.to_vec());
    TERA.render("index", &context)
        .and_then(|result| Ok(HttpResponse::Ok().content_type("text/html").body(result)))
        .map_err(|_| actix_web::error::ErrorNotFound("Render failed"))
}

fn wordgame_main(_: &HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(crate::statics::wordgame::INDEX_HTML)
}

fn wordgame_css(_: &HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/css")
        .body(crate::statics::wordgame::MAIN_CSS)
}

fn wordgame_js(_: &HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/javascript")
        .body(crate::statics::wordgame::MAIN_JS)
}

fn demos(_: &HttpRequest) -> Result<HttpResponse> {
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

fn projects(_: &HttpRequest) -> Result<HttpResponse> {
    let mut context = ::tera::Context::default();
    context.insert("projects", &[(); 1]);
    TERA.render("projects", &context)
        .and_then(|result| Ok(HttpResponse::Ok().content_type("text/html").body(result)))
        .map_err(|_| actix_web::error::ErrorNotFound("Render failed"))
}

fn favicon(_req: &HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("image/png")
        .body(actix_web::Binary::Slice(
            crate::statics::images::FUZEN_CAFE_FAVICON,
        ))
}

fn css(_req: &HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("text/css")
        .body(crate::statics::STYLE_SHEET)
}

fn img(_req: &HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("image/png")
        .body(actix_web::Binary::Slice(
            crate::statics::images::FUZEN_INFO_FAVICON,
        ))
}
