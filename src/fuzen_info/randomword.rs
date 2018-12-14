use rand::prelude::SliceRandom;
use rayon::prelude::*;
static RAND_WORDS: &str = include_str!("words.txt");

#[derive(Deserialize, Copy, Clone)]
pub struct RandomWord {
    #[serde(default)]
    min: usize,
    #[serde(default)]
    max: usize,
    #[serde(default)]
    length: usize,
    #[serde(default = "default_count")]
    count: usize,
}

fn default_count() -> usize {
    1
}

impl RandomWord {
    pub fn gen(self) -> Vec<String> {
        RAND_WORDS
            .par_lines()
            .filter(|word| {
                let len = word.len();
                if self.length != 0 {
                    return len == self.length;
                }
                if self.max != 0 {
                    return (len >= self.min) & (len <= self.max);
                }
                if self.min != 0 {
                    return len >= self.min;
                }
                true
            })
            .collect::<Vec<&str>>()
            .choose_multiple(&mut ::rand::thread_rng(), self.count)
            .map(|l| l.to_string())
            .collect()
    }
}

pub fn randomword(query: ::actix_web::Query<RandomWord>) -> String {
    query.into_inner().gen().join("\n")
}
