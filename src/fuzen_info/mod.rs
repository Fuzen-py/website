mod fractal;
mod fun;
mod randomword;
mod totp;
use actix_web::{http, HttpRequest, HttpResponse};
fn index(_req: &HttpRequest) -> &'static str {
    "Hello World!"
}

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
        .resource("/", |r| r.f(index))
        .resource("/totp", |r| r.method(http::Method::GET).with(totp::totp))
        .resource("/randword", |r| {
            r.method(http::Method::GET).with(randomword::randomword)
        })
        .resource("/fractal.png", |r| r.f(fractal::fractal_png))
        .scope("/hello", |scope| {
            scope
                .resource("", |r| r.method(http::Method::GET).with(fun::hello))
                .resource("/", |r| r.method(http::Method::GET).with(fun::hello))
                .resource("/{name}", |r| r.method(http::Method::GET).with(fun::hello))
        })
        .scope("/baka", |scope| {
            scope
                .resource("", |r| r.method(http::Method::GET).with(fun::baka))
                .resource("/", |r| r.method(http::Method::GET).with(fun::baka))
                .resource("/{name}", |r| r.method(http::Method::GET).with(fun::baka))
        })
        .resource("/favicon.ico", |r| r.f(favicon))
}
