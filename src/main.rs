use rusqlite::Connection;

mod model;
mod persistence;
mod cli;

fn main() {
    cli::repl::repl()
}

