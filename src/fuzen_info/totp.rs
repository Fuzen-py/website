#[derive(Clone, Debug, Deserialize)]
pub struct TOTP {
    pub(crate) token: String,
    #[serde(default)]
    pub(crate) initial_time: u64,
    #[serde(default = "default_period")]
    pub(crate) period: u32,
    #[serde(default = "default_base")]
    pub(crate) base: Vec<u8>,
    #[serde(default = "default_len")]
    pub(crate) length: usize,
}

fn default_base() -> Vec<u8> {
    vec![48, 49, 50, 51, 52, 53, 54, 55, 56, 57]
}
const fn default_len() -> usize {
    6
}

const fn default_period() -> u32 {
    30
}

#[derive(Serialize)]
pub struct TOTPCode {
    code: String,
}

impl std::fmt::Display for TOTPCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.code)
    }
}

impl TOTP {
    fn gen(self) -> Result<TOTPCode, boringauth::oath::ErrorCode> {
        Ok(::boringauth::oath::TOTPBuilder::new()
            .base32_key(&self.token)
            .period(self.period)
            .initial_time(self.initial_time)
            .output_base(&self.base)
            .output_len(self.length)
            .finalize()?
            .generate())
        .and_then(|code| Ok(TOTPCode { code }))
    }
    pub fn route(form: actix_web::Query<Self>) -> String {
        form.into_inner()
            .gen()
            .and_then(|code| Ok(code.to_string()))
            .unwrap_or_else(|_| String::from("Error"))
    }
}
