mod data;
use actix_web::{HttpRequest, HttpResponse, Result};
lazy_static! {
    pub static ref TERA: ::tera::Tera = {
        let mut tera = ::tera::Tera::default();
        tera.add_raw_templates(vec![
            ("base", crate::statics::templates::BASE_TERA),
            ("demos", crate::statics::templates::DEMOS_TERA),
            ("index", crate::statics::templates::INDEX_TERA),
            ("projects", crate::statics::templates::PROJECTS_TERA),
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
        .middleware(::actix_web::middleware::Logger::new(
            "%a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %D",
        ))
}

fn index(_: &HttpRequest) -> Result<HttpResponse> {
    let mut context = ::tera::Context::default();
    context.insert("blog", &data::DEMO_BLOGS.to_vec());
    TERA.render("index", &context)
        .and_then(|result| Ok(HttpResponse::Ok().content_type("text/html").body(result)))
        .map_err(|_| actix_web::error::ErrorNotFound("Render failed"))
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
