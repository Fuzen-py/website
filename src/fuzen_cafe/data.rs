pub static DISCORD_FUZENCAFE_REDIRECT_URI: &str = "https%3A%2F%2Ffuzen.cafe%2Fprofile";
use std::ops::Add;

lazy_static::lazy_static! {
    pub static ref DISCORD_CLIENT_ID: Option<String> = std::env::var("DISCORD_CLIENT_ID").ok();
    pub static ref DISCORD_CLIENT_SECRET: Option<String> =
        std::env::var("DISCORD_CLIENT_SECRET").ok();
}

pub(crate) fn get_discord_client() -> ::std::result::Result<(String, String), failure::Error> {
    Ok((
        std::env::var("DISCORD_CLIENT_ID")?,
        std::env::var("DISCORD_CLIENT_SECRET")?,
    ))
}
pub fn discord_is_configured() -> bool {
    get_discord_client().is_ok()
}

#[derive(serde::Serialize)]
pub struct Demo {
    pub name: String,
    pub link: String,
    pub description: String,
    pub src: Option<String>,
}

#[derive(serde::Serialize, Clone)]
pub struct Blog {
    pub uuid: String,
    pub title: String,
    pub published: ::chrono::DateTime<::chrono::Utc>,
    pub updated: ::chrono::DateTime<::chrono::Utc>,
    pub num: usize,
}

lazy_static::lazy_static! {
    #[derive(serde::Serialize, Clone)]
    pub static ref DEMO_BLOGS: Vec<Blog> = {
        (0..20).map(|n|
            Blog {
                uuid: String::from("NYANCOPTER"),
                title: String::from("NYANCOPTER"),
                published: ::chrono::Utc::now(),
                updated: ::chrono::Utc::now(),
                num: n
            }).collect()
    };
}

// http://discordapp.com/api/users/@me
// Bearer <TOKEN>
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct DiscordInfo {
    pub username: String,
    pub discriminator: String,
    pub mfa_enabled: bool,
    pub id: String,
    pub avatar: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct ResultToken {
    access_token: String,
    token_type: String,
    expires_in: i64,
    pub scope: String,
}

impl ResultToken {
    #[allow(clippy::ptr_arg)]
    pub fn fetch(code: String) -> Option<Self> {
        let (client_id, client_secret) = get_discord_client().unwrap();
        // https://discordapp.com/api/oauth2/token?grant_type=authorization_code&code={code}&redirect_uri={redirect_uri}
        let uri = String::from(
            "https://discordapp.com/api/oauth2/token?grant_type=client_credentials&code=",
        )
        .add(&code)
        .add("&redirect_uri=")
        .add(DISCORD_FUZENCAFE_REDIRECT_URI)
        .add("&client_id=")
        .add(&client_id)
        .add("&client_secret=")
        .add(&client_secret)
        .add("&scope=identify");
        if let Ok(mut res) = reqwest::Client::new()
            .post(&uri)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body("")
            .send()
        {
            match res.json() {
                Ok(token) => Some(token),
                Err(_) => {
                    return None;
                }
            }
        } else {
            None
        }
    }
}

impl std::convert::Into<Token> for ResultToken {
    fn into(self) -> Token {
        Token {
            token: self.access_token,
            expiration: ::std::time::SystemTime::now()
                + ::std::time::Duration::from_secs(self.expires_in as u64),
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Token {
    pub token: String,
    pub expiration: ::std::time::SystemTime,
}

impl Token {
    pub fn discord_info(&self) -> ::std::result::Result<DiscordInfo, failure::Error> {
        let mut res = reqwest::Client::new()
            .get("https://discordapp.com/api/users/@me")
            .header("Authorization", format!("Bearer {}", self.token))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body("")
            .send()?;
        Ok(res.json()?)
    }
    pub fn expired(&self) -> bool {
        ::std::time::SystemTime::now() >= self.expiration
    }
}
