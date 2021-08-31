#![allow(dead_code)]
#![allow(warnings, unused)]

use std::process::exit;
pub mod rulox;
use rulox::CliRulox;

fn main() {
    let mut clirulox = CliRulox::new();

    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        len if len > 2 => {
            println!("Usage: rulox [script]");
            exit(64);
        }
        len if len == 2 => {
            clirulox.run_file(&args[1]);
        }
        _ => {
            clirulox.run_prompt();
        }
    }
}
