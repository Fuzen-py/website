use crate::fuzen_cafe::{data, TERA};
use actix_web::{middleware::session::RequestSession, HttpRequest, HttpResponse};

// TODO: Optimize this
pub fn profile(req: &HttpRequest) -> HttpResponse {
    if !crate::fuzen_info::discord_is_configured() {
        println!("Discord not configured");
        return HttpResponse::NotFound().finish();
    }
    let query = req.query();
    let token = query.get("token");
    let expires = query.get("expires").and_then(|e| e.parse::<u64>().ok());
    if token.is_some() & expires.is_some() {
        let token = token.unwrap();
        let expiration = ::std::time::UNIX_EPOCH + std::time::Duration::from_secs(expires.unwrap());
        let token = data::Token {
            token: token.to_owned(),
            expiration,
        };
        if let Ok(ref discord) = token.discord_info() {
            req.session().set("token", token).is_ok(); // TODO: Handle Error
            req.session().set("expiration", expiration).is_ok(); // TODO: Handle Error
            return render_profile(discord);
        }
    }
    let token = req.session().get::<String>("token").unwrap_or(None);
    let expiration = req
        .session()
        .get::<::std::time::SystemTime>("token")
        .unwrap_or(None);
    if token.is_some() & expiration.is_some() {
        let token = data::Token {
            token: token.unwrap(),
            expiration: expiration.unwrap(),
        };
        if token.expiration <= ::std::time::UNIX_EPOCH {
            render_login(Some("Session Expired"))
        } else if let Ok(ref discord) = token.discord_info() {
            if discord.id == "134090963395149824" {
                render_profile(discord)
            } else {
                render_login(Some("Not allowed"))
            }
        } else {
            render_login(Some("Invalid Login"))
        }
    } else {
        render_login(None)
    }
}

fn render_login(error: Option<&'static str>) -> HttpResponse {
    let mut context = ::tera::Context::default();
    if let Some(ref error) = error {
        context.insert("error", error);
    }
    if let Ok(html) = TERA.render("login", &context) {
        HttpResponse::Ok().content_type("text/html").body(html)
    } else {
        HttpResponse::NotFound().finish()
    }
}

fn render_profile(discord: &data::DiscordInfo) -> HttpResponse {
    let mut context = ::tera::Context::default();
    context.insert("discord", discord);
    if let Ok(html) = TERA.render("profile", &context) {
        HttpResponse::Ok().content_type("text/html").body(html)
    } else {
        HttpResponse::NotFound().finish()
    }
}
