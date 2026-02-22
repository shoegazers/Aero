use rand::{Rng, RngExt, rng};

#[derive(Debug, Default, Clone)]
pub struct Test {
    pub words: Vec<String>,
    pub word_count: i32,
    pub completed: bool,
    pub started: bool
}

pub fn generate_test(count: i32) -> Test {
    let random_words: Vec<String> = vec![
        "possible".to_string(),
        "new".to_string(),
        "public".to_string(),
        "school".to_string(),
        "create".to_string(),
        "dance".to_string(),
        "side".to_string(),
        "design".to_string(),
        "raid".to_string(),
        "deer".to_string(),
        "conscience".to_string(),
        "drop".to_string(),
        "charge".to_string(),
        "change".to_string(),
        "brave".to_string(),
        "integrated".to_string(),
        "practical".to_string(),
        "mean".to_string(),
        "stream".to_string(),
        "which".to_string(),
        "under".to_string(),
        "around".to_string(),
        "troop".to_string(),
        "possible".to_string(),
        "week".to_string(),
        "latest".to_string(),
        "early".to_string(),
        "variant".to_string(),
        "location".to_string(),
        "consider".to_string(),
        "marathon".to_string(),
        "shock".to_string(),
        "deficiency".to_string(),
        "bounce".to_string(),
        "prison".to_string(),
    ];

    let mut words: Vec<String> = Vec::new();
    for i in 0..count {
        words.push(random_words[rand::random_range(0..random_words.len())].clone());
    }
    Test { words: words, word_count: count, completed: false, started: false }
}