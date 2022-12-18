use chrono::{DateTime, Utc};
use crate::model::user::User;

pub struct Quote {
    pub date: DateTime<Utc>,
    pub word: String,
    pub content: String,
    pub user_id: u64,
}

impl Quote {
    pub fn create(user: &User, word: String, content: String) -> Self {
        Quote {
            date: Utc::now(),
            word,
            content,
            user_id: user.id,
        }
    }
}