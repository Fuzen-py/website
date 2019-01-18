#[derive(Serialize)]
pub struct Demo {
    pub name: String,
    pub link: String,
    pub description: String,
    pub src: Option<String>,
}

#[derive(Serialize, Clone)]
pub struct Blog {
    pub uuid: String,
    pub title: String,
    pub published: ::chrono::DateTime<::chrono::Utc>,
    pub updated: ::chrono::DateTime<::chrono::Utc>,
    pub num: usize,
}

lazy_static! {
    #[derive(Serialize, Clone)]
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
#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordInfo {
    pub username: String,
    pub discriminator: String,
    pub mfa_enabled: bool,
    pub id: String,
    pub avatar: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Token {
    pub token: String,
    pub expiration: std::time::SystemTime,
}

impl Token {
    pub fn discord_info(&self) -> ::std::result::Result<DiscordInfo, failure::Error> {
        let mut res = reqwest::Client::new()
            .get("http://discordapp.com/api/users/@me")
            .header("Authorization", format!("Bearer {}", self.token))
            .send()?;
        Ok(res.json()?)
    }
}
