use crate::fuzen_cafe::{data, TERA};
use actix_web::{middleware::session::RequestSession, HttpRequest, HttpResponse};

// TODO: Optimize this
pub fn profile(req: &HttpRequest) -> HttpResponse {
    if !data::discord_is_configured() {
        debug!("Discord not configured");
        return HttpResponse::NotFound().finish();
    }
    let id: String = data::get_discord_client().unwrap().0;
    let link_ = req
        .url_for(
            "discord_authorize",
            &[id, String::from(data::DISCORD_FUZENCAFE_REDIRECT_URI)],
        )
        .unwrap();
    let link = link_.as_str();
    let query = req.query();
    let token: Option<data::Token> = {
        if let Some(ref action) = query.get("action").and_then(|a| Some(a.to_lowercase())) {
            match action.as_str() {
                "login" | "logout" => return render_login(&link, None),
                _ => None,
            }
        } else if let Some(token) = query.get("code").and_then(data::ResultToken::fetch) {
            let t = token.into();
            if req.session().set("token", &t).is_err() {
                return render_login(&link, Some("internal server error"));
            }
            Some(t)
        } else {
            req.session().get::<data::Token>("token").unwrap_or(None)
        }
    };
    if let Some(token) = token {
        if token.expired() {
            render_login(&link, Some("Session is expired"))
        } else if let Ok(discord) = token.discord_info() {
            render_profile(&discord)
        } else {
            render_login(&link, Some("Invalid Session"))
        }
    } else {
        render_login(&link, None)
    }
}

fn render_login(link: &str, error: Option<&'static str>) -> HttpResponse {
    let mut context = ::tera::Context::default();
    if let Some(ref error) = error {
        context.insert("error", error);
    }
    context.insert("login_link", link);
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
