mod model;
mod persistence;
mod cli;

use rusqlite::{Connection};


fn main() {
    cli::run::run()
}

