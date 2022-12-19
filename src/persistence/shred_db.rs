use std::fmt::format;

use chrono::{DateTime, Utc};
use rusqlite::{Connection, Statement};
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
            content   TEXT NOT NULL,
            note      TEXT DEFAULT \"\",
            date      DATETIME DEFAULT current_timestamp
           )",
        (),
    ).expect("Failed to create table quote");
}

fn query_quotes_stmt(mut stmt: Statement) -> Vec<Quote> {
    let rows = stmt.query_map([], |row| {
        Ok(Quote {
            id: row.get(0)?,
            content: row.get(1)?,
            note: row.get(2)?,
            date: row.get(3)?,
        })
    }).expect("Failed to query quotes");
    let mut res = Vec::new();
    for row in rows {
        res.push(row.expect("Failed to query quotes"))
    }
    res
}

impl ShredDB {
    pub fn new() -> Self {
        let conn = Connection::open(DB_FILE).expect("Failed to open database");
        create_table(&conn);
        ShredDB { conn }
    }

    pub fn insert_quote(&self, content: &str) {
        self.conn.execute(
            "INSERT INTO quote (content) VALUES (?1)",
            (content.trim(), ),
        ).expect("Failed to insert data into table quote");
    }

    pub fn insert_quote_with_note(&self, content: &str, note: &str) {
        self.conn.execute(
            "INSERT INTO quote (content, note) VALUES (?1, ?2)",
            (content.trim(), note.trim()),
        ).expect("Failed to insert data into table quote");
    }
    pub fn query_quotes_limit(&self, num: u64) -> Vec<Quote> {
        let query = format!("{} {}", "SELECT id, content, note, date FROM quote ORDER BY date LIMIT", num);
        let mut stmt = self.conn
            .prepare(query.as_str())
            .expect("Failed to prepare statement for query");
        query_quotes_stmt(stmt)
    }

    pub fn query_quotes(&self) -> Vec<Quote> {
        let mut stmt = self.conn
            .prepare("SELECT id, content, note, date FROM quote ORDER BY date")
            .expect("Failed to prepare statement for query");
        query_quotes_stmt(stmt)
    }
}