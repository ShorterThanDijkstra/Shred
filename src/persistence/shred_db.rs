use dir::home_dir;
use rusqlite::{Connection, Statement};

use crate::model::quote::Quote;

const DB_FILE: &str = ".local/share/shred/shred.db";

pub struct ShredDB {
    conn: Connection,
}

fn create_table(conn: &Connection) {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS quote (
            id        INTEGER PRIMARY KEY,
            content   TEXT NOT NULL,
            date      DATETIME DEFAULT current_timestamp
           )",
        (),
    )
    .unwrap();
}

fn query_quotes_stmt(mut stmt: Statement) -> Vec<Quote> {
    let rows = stmt
        .query_map([], |row| {
            Ok(Quote {
                id: row.get(0)?,
                content: row.get(1)?,
                date: row.get(2)?,
            })
        })
        .unwrap();
    let mut res = Vec::new();
    for row in rows {
        res.push(row.unwrap())
    }
    res
}

impl ShredDB {
    pub fn new() -> Self {
        let path = format!("{}/{}", home_dir().unwrap().to_str().unwrap(), DB_FILE);
        let conn = Connection::open(path).unwrap();
        create_table(&conn);
        ShredDB { conn }
    }

    pub fn insert_quote(&self, content: &str) {
        self.conn
            .execute("INSERT INTO quote (content) VALUES (?1)", (content.trim(),))
            .unwrap();
    }

    pub fn query_quotes_limit(&self, num: u64) -> Vec<Quote> {
        let query = format!(
            "{} {}",
            "SELECT id, content, date FROM quote ORDER BY date LIMIT", num
        );
        let stmt = self.conn.prepare(query.as_str()).unwrap();
        query_quotes_stmt(stmt)
    }

    pub fn query_quotes(&self) -> Vec<Quote> {
        let stmt = self
            .conn
            .prepare("SELECT id, content, date FROM quote ORDER BY date")
            .unwrap();
        query_quotes_stmt(stmt)
    }

    pub fn delete(&self, id: u64) {
        let sql = "DELETE FROM quote WHERE id = (?1)";
        self.conn.execute(sql, (id,)).unwrap();
    }

    pub fn update(&self, id: u64, new_content: &str) {
        let sql = "UPDATE quote SET content = (?1) WHERE id = (?2)";
        self.conn.execute(sql, (new_content, id, )).unwrap();
    }
}
