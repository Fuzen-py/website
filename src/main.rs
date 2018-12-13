#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
extern crate actix_web; //Rust > 1.31
extern crate boringauth; //Rust > 1.31
extern crate chrono; //Rust > 1.31
extern crate image; //Rust > 1.31
extern crate num; //Rust > 1.31
extern crate pretty_env_logger;
extern crate rand; //Rust > 1.31
extern crate rayon; //Rust > 1.31
extern crate tera; //Rust > 1.31
use actix_web::server;

mod fuzen_cafe;
mod fuzen_info;
mod hosts;
mod statics;

fn log(app: actix_web::App) -> actix_web::App {
    app.middleware(actix_web::middleware::Logger::new(
        "%{HOST}i: %a \"%r\" %s %b \"%{Referer}i\" \"%{User-Agent}i\" %D",
    ))
}

fn main() {
    pretty_env_logger::init();
    let addr: String =
        std::env::var("WEB_LISTEN_ADDR").unwrap_or_else(|_| String::from("127.0.0.1:8080"));
    server::new(|| vec![log(fuzen_info::route()), log(fuzen_cafe::route())])
        .bind(addr)
        .unwrap()
        .run()
}
