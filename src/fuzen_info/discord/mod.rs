use actix_web::{http, HttpRequest, HttpResponse, Query, Result};
use std::ops::Add;

lazy_static! {
    pub static ref DISCORD_CLIENT_ID: Option<String> = std::env::var("DISCORD_CLIENT_ID").ok();
    pub static ref DISCORD_CLIENT_SECRET: Option<String> =
        std::env::var("DISCORD_CLIENT_SECRET").ok();
}

fn get_discord_client() -> ::std::result::Result<(String, String), failure::Error> {
    Ok((
        std::env::var("DISCORD_CLIENT_ID")?,
        std::env::var("DISCORD_CLIENT_SECRET")?,
    ))
}

pub fn discord_is_configured() -> bool {
    get_discord_client().is_ok()
}

static DISCORD_FUZENINFO_REDIRECT_URI: &str = "https%3A%2F%2Ffuzen.info%2Fdiscord";
#[allow(unused)]
static DISCORD_FUZENCAFE_REDIRECT_URI: &str = "https%3A%2F%2Ffuzen.cafe%2Fprofile";

pub fn login(req: &HttpRequest) -> Result<HttpResponse> {
    if let Ok((client_id, client_secret)) = get_discord_client() {
        Ok(HttpResponse::TemporaryRedirect().body(
            req.url_for("discord_authorize", &[client_id, client_secret])?
                .to_string(),
        ))
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}

pub fn callback(query: Query<Code>) -> HttpResponse {
    if let Ok((client_id, client_secret)) = get_discord_client() {
        println!("{:#?}", query);
        let code = query.into_inner().code;
        // https://discordapp.com/api/oauth2/token?grant_type=authorization_code&code={code}&redirect_uri={redirect_uri}
        let uri = String::from(
            "https://discordapp.com/api/oauth2/token?grant_type=client_credentials&code=",
        )
        .add(&code)
        .add("&redirect_uri=")
        .add(DISCORD_FUZENINFO_REDIRECT_URI)
        .add("&client_id=")
        .add(&client_id)
        .add("&client_secret=")
        .add(&client_secret)
        .add("&scope=identify");
        println!("Authorizing");
        if let Ok(mut res) = reqwest::Client::new()
            .post(&uri)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body("")
            .send()
        {
            println!("Tokenizing..");
            let token: Token = match res.json() {
                Ok(token) => token,
                Err(e) => {
                    println!("{:#?}", e);
                    return HttpResponse::Ok()
                        .content_type("text")
                        .body("Failed to get token");
                }
            };
            let mut resp = HttpResponse::TemporaryRedirect()
                .header(
                    "location",
                    format!(
                        "https://fuzen.cafe/login?token={}&expires={}",
                        token.access_token, token.expires_in
                    ),
                )
                .finish();
            if let Err(e) = token.cookie(&mut resp) {
                println!("{:#?}", e);
            }
            return resp;
        }
        println!("Failed");
        // Make a request to uri
        // Type: Post
        // Basic: ${CLIENT_ID}:${CLIENT_SECRET}
        // Response: `Token`
        // Get User ID from `DiscordInfo` & generate an expering session token

        HttpResponse::Ok()
            .content_type("text")
            .body("Failed to get token")
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Code {
    code: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    access_token: String,
    token_type: String,
    expires_in: i64,
    scope: String,
}

impl Token {
    fn cookie(self, res: &mut HttpResponse) -> Result<()> {
        let cookie = http::Cookie::build("token", self.access_token)
            .max_age(time::Duration::seconds(self.expires_in))
            .secure(true)
            .finish();
        res.add_cookie(&cookie)?;
        Ok(())
    }
}

// TODO: Add refresh token

// TODO: Implement this as part of the account creation process
