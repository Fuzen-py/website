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
