use crate::model::quote::Quote;
use crate::persistence::shred_db::*;

pub fn run() {
    let shred = ShredDB::new();

    shred.insert_quote(&Quote::create("hello", "hello world"));

    for quote in shred.query_quotes() {
        println!("{:?}", quote)
    }
}