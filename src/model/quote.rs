use std::fmt::{Display, Formatter};

use chrono::{DateTime, Utc};

pub struct Quote {
    pub id: u64,
    pub content: String,
    pub date: DateTime<Utc>,
}

impl Display for Quote {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}@{}] {}", self.id, self.date.date_naive(), self.content)
    }
}