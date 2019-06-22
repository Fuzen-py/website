mod fuzen_cafe;
mod fuzen_info;
mod hosts;
mod statics;

use self::fuzen_cafe::{
    FuzenCafeRoutes, Images as CafeImages, Statics as CafeStatics, WordGames as CafeWordGame,
};
use self::fuzen_info::FuzenInfoRoutes;

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    pretty_env_logger::init();
    let addr: String =
        std::env::var("WEB_LISTEN_ADDR").unwrap_or_else(|_| String::from("127.0.0.1:8080"));
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .wrap(FuzenCafeRoutes::ident_service())
            // Shared (Fuzen.info & Fuzen.cafe)
            .service(
                actix_web::web::resource("/")
                    .route(FuzenInfoRoutes::Help.into())
                    .route(FuzenCafeRoutes::Index.into()),
            )
            .service(
                actix_web::web::resource("/favicon")
                    .route(FuzenInfoRoutes::Favicon.into())
                    .route(FuzenCafeRoutes::Static(CafeStatics::Favicon).into()),
            )
            // Fuzen.info
            .route("/totp", FuzenInfoRoutes::Totp.into())
            .route("/randomword", FuzenInfoRoutes::RandomWord.into())
            .route("/fractal.png", FuzenInfoRoutes::Fractal.into())
            .route("/hello/{name}", FuzenInfoRoutes::Hello.into())
            .route("/baka/{name}", FuzenInfoRoutes::Baka.into())
            .route("/help", FuzenInfoRoutes::Help.into())
            .route("/help/{route}", FuzenInfoRoutes::Help.into())
            .wrap(FuzenInfoRoutes::cors())
            // Fuzen.cafe
            .route("/demos", FuzenCafeRoutes::Demos.into())
            .route("/projects", FuzenCafeRoutes::Projects.into())
            .route(
                "/static/style.css",
                FuzenCafeRoutes::Static(CafeStatics::Style).into(),
            )
            .route(
                "/static/images/FuzenInfo.png",
                FuzenCafeRoutes::Static(CafeStatics::Image(CafeImages::FuzenInfo)).into(),
            )
            .route("/wordgame", FuzenCafeRoutes::WordGame.into())
            .route(
                "/static/wordgame/main.css",
                FuzenCafeRoutes::Static(CafeStatics::WordGame(CafeWordGame::MainCSS)).into(),
            )
            .route(
                "/static/wordgame/main.js",
                FuzenCafeRoutes::Static(CafeStatics::WordGame(CafeWordGame::MainJS)).into(),
            )
            .route("/profile", FuzenCafeRoutes::Profile.into())
            .external_resource(
                fuzen_cafe::EXTERNAL_DISCORD_AUTHORIZE.name,
                fuzen_cafe::EXTERNAL_DISCORD_AUTHORIZE.url,
            )
            .external_resource(
                fuzen_cafe::EXTERNAL_DISCORD_TOKEN.name,
                fuzen_cafe::EXTERNAL_DISCORD_TOKEN.url,
            )
    })
    .bind(addr)?
    .run()?;
    //let _cafe = fuzen_cafe::start_server(&addr);

    Ok(())
}
