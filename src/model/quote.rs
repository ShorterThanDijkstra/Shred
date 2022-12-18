use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Quote {
    pub id: u64,
    // pub date: Option<DateTime<Utc>>,
    pub word: String,
    pub content: String,
}

impl Quote {
    pub fn create(word: &str, content: &str) -> Self {
        Quote {
            id: 0,
            // date: None,
            word: word.to_string(),
            content: content.to_string(),
        }
    }
}