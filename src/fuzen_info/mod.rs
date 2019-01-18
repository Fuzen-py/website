mod fractal;
mod fun;
mod help;
mod randomword;
mod totp;
use actix_web::{http, HttpRequest, HttpResponse};

fn favicon(_req: &HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .content_type("image/png")
        .body(actix_web::Binary::Slice(
            crate::statics::images::FUZEN_INFO_FAVICON,
        ))
}

pub fn route() -> ::actix_web::App {
    crate::hosts::Hosts::FuzenInfo
        .filter(::actix_web::App::new())
        .resource("/", |r| r.method(http::Method::GET).with(help::help))
        .resource("/totp", |r| {
            r.method(http::Method::GET).with(totp::TOTP::route)
        })
        .resource("/randomword", |r| {
            r.method(http::Method::GET).with(randomword::randomword)
        })
        .resource("/fractal.png", |r| r.f(fractal::fractal_png))
        .resource("/hello/{name}", |r| {
            r.method(http::Method::GET).with(fun::Hello::route)
        })
        .resource("/baka/{name}", |r| {
            r.method(http::Method::GET).with(fun::Baka::route)
        })
        .resource("/favicon.ico", |r| r.f(favicon))
        .scope("/help", |scope| {
            scope
                .resource("", |r| r.method(http::Method::GET).with(help::help))
                .resource("/", |r| r.method(http::Method::GET).with(help::help))
                .resource("/{route}", |r| r.method(http::Method::GET).with(help::help))
        })
        .middleware(actix_web::middleware::cors::Cors::default())
}
