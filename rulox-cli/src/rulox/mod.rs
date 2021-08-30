extern crate rulox_core;

use rulox_core::rulox::{
    interpreter::Interpreter, parser::Parser, resolver::Resolver, scanner::Scanner, token::Token,
};

use std::fs;
use std::io::{self, Read, Write};
use std::process::exit;

///////////////////////////////////
// use self::parser::Parser;     //
// use self::resolver::Resolver; //
// use self::scanner::Scanner;   //
// use self::token::Token;       //
///////////////////////////////////

pub struct Rulox {
    had_errors: bool,
}

impl Rulox {
    pub fn new() -> Rulox {
        Rulox { had_errors: false }
    }

    // Open file with given path, read_to_end into mutable data vector
    // Data is a bytes vector &[u8]
    pub fn run_file(&mut self, path: &str) {
        // let mut file = File::open(path).expect("No file found.");

        // let mut data = Vec::new();
        // file.read_to_end(&mut data).expect("Unable to read file.");
        let source = fs::read_to_string(path).expect("Unable to read file.");

        println!("Opening file {}...", path);

        let mut interpreter = Interpreter::new();

        self.run(source, &mut interpreter);

        if self.had_errors {
            exit(65);
        };
    }

    // Input code taken from std rust documentation for Stdin::read_line
    // Needs to take a '&mut self' because modifying the had_errors field
    pub fn run_prompt(&mut self) {
        let mut input = String::new();
        let mut interpreter = Interpreter::new();

        loop {
            print!("> ");

            // Need to flush stdout before reading a new line, otherwise `>` is prepended to input
            io::stdout().flush().unwrap();

            // if successful read_line returns total number of bytes read
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    // To report bytes, replace _ with n for pattern
                    // println!("{} bytes read", n);
                    self.run(input.clone(), &mut interpreter);
                }
                Err(error) => println!("error: {}", error),
            }

            // Convert input String to bytes and pass to run

            // Since read_line appends to buffer, need to clear
            input.clear();

            // Reset error flag in interactive loop
            self.had_errors = false;
        }
    }

    // Brains of the operation, corse function that parses a line of bytes
    fn run(&mut self, source: String, interpreter: &mut Interpreter) {
        if self.had_errors {
            exit(65);
        };

        let scanner = Scanner::new(source, self);
        let tokens: Vec<Token> = scanner.scan_tokens();

        let mut parser = Parser::new(tokens, self);
        let statements: Vec<Stmt> = parser.parse();

        let mut resolver = Resolver::new(interpreter);
        resolver.resolve_source(&statements);

        if self.had_errors {
            return;
        }

        interpreter.interpret(statements);
    }

    pub fn error_line(&mut self, line: u32, msg: String) {
        self.report(line, "", msg);
    }

    pub fn error_token(&mut self, token: &Token, msg: String) {
        self.report(token.line, &format!("at '{}'", token.t_type), msg);
    }

    // TODO: Implement RuntimeError
    pub fn runtime_error(&mut self, error_msg: String) {
        println!("Runtime error: {}", error_msg);
        self.had_errors = true;
    }

    fn report(&mut self, line: u32, ubi: &str, msg: String) {
        println!("[Line {}] Error {}: {}", line, ubi, msg);
        self.had_errors = true;
    }
}
