use rand::prelude::SliceRandom;
use rayon::prelude::*;
static RAND_WORDS: &str = include_str!("words.txt");

#[derive(serde::Deserialize, Copy, Clone)]
pub struct RandomWordQuery {
    #[serde(default)]
    min: usize,
    #[serde(default)]
    max: usize,
    #[serde(default)]
    length: usize,
    #[serde(default = "default_count")]
    count: usize,
}

const fn default_count() -> usize {
    1
}

impl RandomWordQuery {
    fn inner_gen(self) -> Vec<String> {
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
    fn gen(self) -> RandomWord {
        RandomWord {
            words: self.inner_gen(),
        }
    }
}

#[derive(serde::Serialize)]
struct RandomWord {
    words: Vec<String>,
}

impl std::fmt::Display for RandomWord {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.words.join("\n"))
    }
}

pub fn randomword(query: ::actix_web::web::Query<RandomWordQuery>) -> String {
    query.into_inner().gen().to_string()
}
