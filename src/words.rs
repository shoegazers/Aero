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
        "rotate".to_string()
    ];

    let mut words: Vec<String> = Vec::new();
    for i in 0..count {
        words.push(random_words[rand::random_range(0..random_words.len())].clone());
    }
    Test { words: words, word_count: count, completed: false, started: false }
}