use wasm_bindgen::prelude::*;

// Structure:
// This contains a rulox-wasm struct
// Calls the public API of the Rulox struct
// Serializes to JSON

#[wasm_bindgen]
pub struct Rulox {
    had_errors: bool,
    source: String,
    // environments: String,
    // parse_tree: Vec<Stmt>,
    // token_stream: Vec<Token>,
    // error_msg: Vec<String>,
}

#[wasm_bindgen]
impl Rulox {
    pub fn new(source: String) -> Rulox {
        Rulox {
            had_errors: false,
            source,
            // environments: String::new(),
            // token_stream: Vec::new(),
            // parse_tree: Vec::new(),
            // error_msg: Vec::new(),
        }
    }

    // pub fn tokens(&mut self) -> JsValue {
    //     self.token_stream = self.tokenize();

    //     JsValue::from_serde(&self.token_stream).unwrap()
    // }

    // pub fn parse_tree(&mut self) -> JsValue {
    //     self.parse_tree = self.parse();

    //     JsValue::from_serde(&self.parse_tree).unwrap()
    // }

    // pub fn interpret(&mut self) -> JsValue {
    //     let output = self.run();

    //     JsValue::from_serde(&output).unwrap()
    // }

    // pub fn had_errors(&self) -> bool {
    //     self.had_errors
    // }

    // fn run(&mut self) -> Vec<String> {
    //     let mut interpreter = Interpreter::new();

    //     let mut resolver = Resolver::new(&mut interpreter);

    //     let mut output = Vec::new();
    //     match resolver.resolve_source(&self.parse_tree) {
    //         Err(e) => output.push(format!("Resolver error: {:?}", e)),
    //         _ => {}
    //     }

    //     if self.had_errors {
    //         output.append(&mut self.error_msg);
    //         return output;
    //     }

    //     let mut result = interpreter.interpret(self.parse_tree.clone());
    //     output.append(&mut result);
    //     self.environments = interpreter.get_environment();

    //     if output.len() == 0 {
    //         output.push(String::from("No output to display."));
    //     }

    //     output
    // }

    // pub fn get_environment(&self) -> String {
    //     self.environments.clone()
    // }

    // fn tokenize(&mut self) -> Vec<Token> {
    //     let scanner = Scanner::new(self.source.clone(), self);
    //     scanner.scan_tokens()
    // }

    // fn parse(&mut self) -> Vec<Stmt> {
    //     let mut parser = Parser::new(self.token_stream.clone(), self);
    //     parser.parse()
    // }

    // fn error_line(&mut self, line: usize, msg: String) {
    //     self.report((line, 0), "", msg);
    // }

    // fn error_token(
    //     &mut self,
    //     token_type: String,
    //     token_line: usize,
    //     token_col: usize,
    //     msg: String,
    // ) {
    //     self.report(
    //         (token_line, token_col),
    //         &format!("at '{}'", token_type),
    //         msg,
    //     );
    // }

    // // pub fn runtime_error(&mut self, error_msg: String) {
    // //     println!("Runtime error: {}", error_msg);
    // //     self.had_errors = true;
    // // }

    // fn report(&mut self, pos: (usize, usize), ubi: &str, msg: String) {
    //     let (line, col) = pos;
    //     self.error_msg.push(format!(
    //         "[Line {} Col {}] Error {}: {}",
    //         line, col, ubi, msg
    //     ));
    //     self.had_errors = true;
    // }
}
