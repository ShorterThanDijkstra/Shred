use chrono::{DateTime, Utc};
use rusqlite::{Connection};
use rusqlite::types::{FromSql, FromSqlResult, ValueRef};
use crate::model::quote::Quote;

const DB_FILE: &str = "shred.db";

pub struct ShredDB {
    conn: Connection,
}

fn create_table(conn: &Connection) {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS quote (
            id        INTEGER PRIMARY KEY,
            word      TEXT NOT NULL,
            content   TEXT NOT NULL,
            date      DATETIME DEFAULT current_timestamp
           )",
        (),
    ).expect("Failed to create table quote");
}

impl ShredDB {
    pub fn new() -> Self {
        let conn = Connection::open(DB_FILE).expect("Failed to open database");
        create_table(&conn);
        ShredDB { conn }
    }

    pub fn insert_quote(&self, quote: &Quote) {
        self.conn.execute(
            "INSERT INTO quote (word, content) VALUES (?1, ?2)",
            (&quote.word, &quote.content),
        ).expect("Failed to insert data into table quote");
    }

    pub fn query_quotes(&self) -> Vec<Quote> {
        let mut stmt = self.conn
            .prepare("SELECT id, word, content, date FROM quote")
            .expect("Failed to prepare statement for query");
        let rows = stmt.query_map([], |row| {
            Ok(Quote {
                id: row.get(0)?,
                word: row.get(1)?,
                // date: Some(row.get(2)?),
                content: row.get(3)?,
            })
        }).expect("Failed to query quotes");
        let mut res = Vec::new();
        for row in rows {
            res.push(row.expect("Failed to query quotes"))
        }
        res
    }
}