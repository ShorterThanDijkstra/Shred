use crate::model::quote::Quote;
use crate::persistence::shred_db::ShredDB;
use chrono::{DateTime, Utc};
use dir::home_dir;
use rusqlite::{Connection, Statement};
use std::{
    fs::{self, File},
    time::SystemTime,
};
const DB_FILE: &str = ".local/share/shred/shred.db";

pub struct SQLiteDB {
    conn: Connection,
}

impl SQLiteDB {
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
    pub fn new() -> impl ShredDB {
        let path = format!("{}/{}", home_dir().unwrap().to_str().unwrap(), DB_FILE);
        let conn = Connection::open(path).unwrap();
        Self::create_table(&conn);
        SQLiteDB { conn }
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
}
impl ShredDB for SQLiteDB {
    fn insert_quote(&self, content: &str) {
        self.conn
            .execute("INSERT INTO quote (content) VALUES (?1)", (content.trim(),))
            .unwrap();
    }

    fn query_quotes_limit(&self, num: u64) -> Vec<Quote> {
        let query = format!(
            "{} {}",
            "SELECT id, content, date FROM quote ORDER BY date LIMIT", num
        );
        let stmt = self.conn.prepare(query.as_str()).unwrap();
        Self::query_quotes_stmt(stmt)
    }

    fn query_quotes(&self) -> Vec<Quote> {
        let stmt = self
            .conn
            .prepare("SELECT id, content, date FROM quote ORDER BY date")
            .unwrap();
        Self::query_quotes_stmt(stmt)
    }

    fn delete(&self, id: u64) {
        let sql = "DELETE FROM quote WHERE id = (?1)";
        self.conn.execute(sql, (id,)).unwrap();
    }

    fn update(&self, id: u64, new_content: &str) {
        let sql = "UPDATE quote SET content = (?1) WHERE id = (?2)";
        self.conn.execute(sql, (new_content, id)).unwrap();
    }
    fn backup(&self) {
        let time: DateTime<Utc> = SystemTime::now().into();
        let to_file = format!("{}-backup-{}", DB_FILE, time.format("%d%m%Y%T"));
        println!("{}",to_file);
        let from_path = format!("{}/{}", home_dir().unwrap().to_str().unwrap(), DB_FILE);
        let to_path = format!("{}/{}", home_dir().unwrap().to_str().unwrap(), to_file);
        File::create(&to_path).unwrap();
        fs::copy(&from_path, &to_path).unwrap();
    }
}
