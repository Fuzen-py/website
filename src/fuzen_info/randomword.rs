use rand::prelude::SliceRandom;
use rayon::prelude::*;
static RAND_WORDS: &str = include_str!("words.txt");

fn new(min: usize, max: usize) -> String {
    RAND_WORDS
        .par_lines()
        .filter(|l| ((l.len() >= min) | (min == 0usize)) && ((l.len() <= max) | (max == 0usize)))
        .collect::<Vec<&str>>()
        .choose(&mut ::rand::thread_rng())
        .and_then(|w| Some(w.to_string()))
        .unwrap_or_default()
}

#[derive(Deserialize)]
pub struct RandomWord {
    #[serde(default)]
    min: usize,
    #[serde(default)]
    max: usize,
}

impl std::convert::Into<String> for RandomWord {
    fn into(self) -> String {
        new(self.min, self.max)
    }
}

pub fn randomword(query: ::actix_web::Query<RandomWord>) -> String {
    query.into_inner().into()
}
