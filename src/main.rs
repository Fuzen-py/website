#[macro_use]
extern crate failure;
#[macro_use]
extern crate serde_derive;
extern crate actix_web; //Rust > 1.31
extern crate boringauth; //Rust > 1.31
extern crate image; //Rust > 1.31
extern crate num; //Rust > 1.31
extern crate rand; //Rust > 1.31
extern crate rayon; //Rust > 1.31
use ::actix_web::server;

mod fuzen_info;
mod hosts;

fn main() {
    server::new(fuzen_info::route)
        .bind("127.0.0.1:8080")
        .unwrap()
        .run()
}
