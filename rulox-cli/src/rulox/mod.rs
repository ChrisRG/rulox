use rulox_core::rulox::{
    ast::Stmt, interpreter::Interpreter, parser::Parser, resolver::Resolver, scanner::Scanner,
    token::Token, Rulox,
};

use std::fs;
use std::io::{self, Read, Write};
use std::process::exit;

pub struct CliRulox {
    had_errors: bool,
}

impl CliRulox {
    pub fn new() -> Self {
        Self { had_errors: false }
    }

    pub fn run_file(&mut self, path: &str) {
        let source = fs::read_to_string(path).expect("Unable to read file.");

        println!("Opening file {}...", path);

        self.run(source);

        if self.had_errors {
            exit(65);
        };
    }

    pub fn run_prompt(&mut self) {
        let mut input = String::new();

        loop {
            print!("> ");

            // Need to flush stdout before reading a new line, otherwise `>` is prepended to input
            io::stdout().flush().unwrap();

            match io::stdin().read_line(&mut input) {
                Ok(_) => match input.as_str() {
                    "exit\n" | "quit\n" => {
                        exit(0);
                    }
                    _ => self.run(input.clone()),
                },
                Err(error) => println!("error: {}", error),
            }

            // Since read_line appends to buffer, need to clear
            input.clear();

            // Reset error flag in interactive loop
            self.had_errors = false;
        }
    }

    fn run(&mut self, source: String) {
        let mut rulox = Rulox::new(source);

        if rulox.had_errors {
            self.report_errors(rulox.error_msg);
            exit(65);
        };

        rulox.tokenize();
        rulox.parse();
        let output = rulox.run();

        output.iter().for_each(|line| {
            println!("{}", line);
        });
    }

    fn report_errors(&self, errors: Vec<String>) {
        errors.iter().for_each(|err| {
            println!("{}", err);
        });
    }
}
