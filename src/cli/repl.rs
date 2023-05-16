use io::{stdin, stdout};
use std::io;
use std::io::Write;
use std::process::exit;

use crate::persistence::shred_db::*;

const PROMPT_OUT: &str = "=>";
const PROMPT_IN: &str = ">";

fn output(msg: &str) {
    println!("{} {}", PROMPT_OUT, msg)
}

fn prompt() {
    print!("{} ", PROMPT_IN);
    stdout().flush().unwrap();
}

fn usage() {
    println!("{}\n", "Commands available from the prompt:");
    println!("{:<48} {}", "<quote>", "save a quote");
    println!("{:<48} {}", ":?", "help");
    println!("{:<48} {}", ":(l|list) [<number>]", "list quotes (with an optional number for limit)");
    println!("{:<48} {}",":(e|edit) <id>", "edit a quote");
    println!("{:<48} {}",":(d|delete) <id>", "delete a quote");
    println!("{:<48} {}", ":(q|quit)", "quit");
}

fn eval_cmd(cmd_raw: &str, shred: &ShredDB) {
    let split: Vec<&str> = cmd_raw.split_whitespace().collect();

    if split.is_empty() {
        usage();
        return;
    }
    match split[0] {
        "l" | "list" => {
            match split.len() {
                1 => {
                    for quote in shred.query_quotes() {
                        output(format!("{}", quote).as_str())
                    }
                }
                2 => {
                    if let Ok(num) = split[1].parse::<u64>() {
                        for quote in shred.query_quotes_limit(num) {
                            output(format!("{}", quote).as_str())
                        }
                    } else { usage() }
                }
                _ => {
                    usage()
                }
            }
        }
        "q" | "quit" => {
            exit(0)
        }
        "d" | "delete" => {
            match split.len() {
                2 => {
                    if let Ok(num) = split[1].parse::<u64>()  {
                        shred.delete(num);
                        output(format!("deleted: {}", num).as_str())

                    } else {
                        usage()
                    }
                }
                _ => {
                    usage()
                }
            }
        }
        "e" | "edit" => {
            match split.len() {
                3 => {
                    if let Ok(num) = split[1].parse::<u64>()  {
                        shred.update(num, split[2]);
                        output(format!("updated: {}", num).as_str())
                    } else {
                        usage()
                    }
                }
                _ => {
                    usage()
                }
            }
        }
        "?" => {
            usage();
        }
        _ => {
            usage();
        }
    }
}

fn eval(input: &str, shred: &ShredDB) {
    match input {
        input if input.starts_with(":") => {
            eval_cmd(&input[1..], shred)
        }

        input if !input.is_empty() => {
            let split: Vec<&str> = input.split("//").collect();
            match split.len() {
                1 => {
                    shred.insert_quote(split[0])
                }
                _ => {
                    usage()
                }
            }
        }
        _ => { usage() }
    }
}

pub fn repl() {
    let shred = ShredDB::new();

    loop {
        prompt();

        let mut content = String::new();
        stdin().read_line(&mut content).unwrap();

        eval(content.trim(), &shred);
    }
}