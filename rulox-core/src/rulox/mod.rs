pub mod ast;
pub mod environment;
pub mod function;
pub mod interpreter;
pub mod parser;
pub mod resolver;
pub mod scanner;
pub mod token;

use ast::Stmt;
use interpreter::Interpreter;
use parser::Parser;
use resolver::Resolver;
use scanner::Scanner;
use token::Token;

pub struct Rulox {
    pub had_errors: bool,
    source: String,
    environments: String,
    parse_tree: Vec<Stmt>,
    token_stream: Vec<Token>,
    pub error_msg: Vec<String>,
}

impl Rulox {
    pub fn new(source: String) -> Rulox {
        Rulox {
            had_errors: false,
            source,
            environments: String::new(),
            token_stream: Vec::new(),
            parse_tree: Vec::new(),
            error_msg: Vec::new(),
        }
    }

    pub fn run(&mut self) -> Vec<String> {
        let mut interpreter = Interpreter::new();

        let mut resolver = Resolver::new(&mut interpreter);

        let mut output = Vec::new();
        if let Err(e) = resolver.resolve_source(&self.parse_tree) {
            output.push(format!("Resolver error: {:?}", e))
        }

        if self.had_errors {
            output.append(&mut self.error_msg);
            return output;
        }

        let mut result = interpreter.interpret(self.parse_tree.clone());
        println!("{:?}", result);
        output.append(&mut result);
        self.environments = interpreter.get_environment();

        if output.is_empty() {
            output.push(String::from("No output to display."));
        }

        output
    }

    pub fn get_environment(&self) -> String {
        self.environments.clone()
    }

    pub fn tokenize(&mut self) {
        let scanner = Scanner::new(self.source.clone(), self);
        self.token_stream = scanner.scan_tokens();
    }

    pub fn parse(&mut self) {
        let mut parser = Parser::new(self.token_stream.clone(), self);
        self.parse_tree = parser.parse()
    }

    fn error_line(&mut self, line: usize, msg: String) {
        self.report((line, 0), "".to_string(), msg);
    }

    fn error_token(
        &mut self,
        token_type: String,
        token_line: usize,
        token_col: usize,
        msg: String,
    ) {
        self.report((token_line, token_col), token_type, msg);
    }

    fn report(&mut self, pos: (usize, usize), t_type: String, msg: String) {
        let (line, col) = pos;
        self.error_msg.push(format!(
            "[error @ {} : {}] \n\t --> `{}` = {}",
            line, col, t_type, msg
        ));
        self.had_errors = true;
    }
}
