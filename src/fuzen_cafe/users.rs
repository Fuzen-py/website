use crate::fuzen_cafe::{data, TERA};
use actix_web::{middleware::session::RequestSession, HttpRequest, HttpResponse};

// TODO: Optimize this
pub fn profile(req: &HttpRequest) -> HttpResponse {
    if !data::discord_is_configured() {
        debug!("Discord not configured");
        return HttpResponse::NotFound().finish();
    }
    let query = req.query();
    let token: Option<data::Token> = {
        if let Some(ref action) = query.get("action").and_then(|a| Some(a.to_lowercase())) {
            match action.as_str() {
                "login" | "logout" => return render_login(None),
                _ => None,
            }
        } else if let Some(token) = query.get("code").and_then(data::ResultToken::fetch) {
            let t = token.into();
            if req.session().set("token", &t).is_err() {
                return render_login(Some("internal server error"));
            }
            Some(t)
        } else {
            req.session().get::<data::Token>("token").unwrap_or(None)
        }
    };
    if let Some(token) = token {
        if token.expired() {
            render_login(Some("Session is expired"))
        } else if let Ok(ref discord) = token.discord_info() {
            render_profile(discord)
        } else {
            render_login(Some("Invalid Session"))
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

// TODO: Add refresh token

// IDEA: Implement this as part of the account creation process instead of just login
