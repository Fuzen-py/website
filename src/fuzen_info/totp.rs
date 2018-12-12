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
    [48, 49, 50, 51, 52, 53, 54, 55, 56, 57].to_vec()
}
fn default_len() -> usize {
    6
}

fn default_period() -> u32 {
    30
}

impl TOTP {
    pub(crate) fn gen(self) -> Result<String, boringauth::oath::ErrorCode> {
        Ok(::boringauth::oath::TOTPBuilder::new()
            .base32_key(&self.token)
            .period(self.period)
            .initial_time(self.initial_time)
            .output_base(&self.base)
            .output_len(self.length)
            .finalize()?
            .generate())
    }
}

pub fn totp(form: actix_web::Query<TOTP>) -> String {
    form.into_inner()
        .gen()
        .unwrap_or_else(|_| String::from("Error"))
}
