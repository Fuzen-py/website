mod fractal;
mod fun;
mod help;
mod randomword;
mod totp;

use crate::hosts::Hosts;
use actix_web::{web, HttpResponse, Result, Route};

fn favicon() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("image/png")
        .body(crate::statics::images::FUZEN_INFO_FAVICON))
}

fn info() -> Route {
    web::get().guard(Hosts::FuzenInfo)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum FuzenInfoRoutes {
    Help,
    Totp,
    RandomWord,
    Fractal,
    Hello,
    Baka,
    Favicon,
}

impl FuzenInfoRoutes {
    pub fn cors() -> actix_cors::CorsFactory {
        actix_cors::Cors::default()
    }
}

impl std::convert::Into<Route> for FuzenInfoRoutes {
    fn into(self) -> Route {
        use FuzenInfoRoutes::*;
        match self {
            Help => info().to_async(help::help),
            Hello => info().to_async(fun::Hello::route),
            Totp => info().to(totp::TOTP::route),
            RandomWord => info().to(randomword::randomword),
            Fractal => info().to_async(fractal::fractal_png),
            Baka => info().to_async(fun::Baka::route),
            Favicon => info().to_async(favicon),
        }
    }
}
