#![allow(dead_code)]
#![allow(warnings, unused)]
// #[allow(clippy::all)]

use std::process::exit;
mod rulox;
use rulox::Rulox;

fn main() {
    let mut rulox = Rulox::new();

    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        len if len > 2 => {
            println!("Usage: rulox [script]");
            exit(64);
        }
        len if len == 2 => {
            rulox.run_file(&args[1]);
        }
        _ => {
            rulox.run_prompt();
        }
    }
}
