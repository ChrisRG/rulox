use super::token::Token;
use super::token::TokenType;
use super::token::TokenType::*;
use super::Rulox;

pub struct Scanner<'a> {
    source: Vec<char>,
    rulox: &'a mut Rulox,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    col: usize,
    line: usize,
}

// Consider reworking source as &'a str and iterating through it with .as_bytes()
impl<'a> Scanner<'a> {
    pub fn new(source_code: String, rulox: &'a mut Rulox) -> Scanner<'a> {
        Scanner {
            rulox,
            source: source_code.chars().collect(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            col: 0,
            line: 1,
        }
    }

    // Entry point for the scanner
    // Loops until end, scans each token, pushs a final EOF token, returns a Vec
    pub fn scan_tokens(mut self) -> Vec<Token> {
        while !self.is_at_end() {
            // Start refers to beginning of each token,
            // Current is a cursor that can look ahead and pick up contents of literals
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            t_type: Eof,
            line: self.line,
            col: self.source.len() - 1,
        });

        self.tokens
    }

    // We're done if we hit the length of the source
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    // Every call of scan_token advances the scanner one byte and tries to match
    // It it matches a TokenType, it adds the token, otherwise flag an error in Rulox
    fn scan_token(&mut self) {
        let c: char = self.advance();
        match c {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            ';' => self.add_token(Semicolon),
            '*' => self.add_token(Star),
            '!' => {
                let t_type = if self.match_next('=') {
                    BangEqual
                } else {
                    Bang
                };
                self.add_token(t_type);
            }
            '=' => {
                let t_type = if self.match_next('=') {
                    EqualEqual
                } else {
                    Equal
                };
                self.add_token(t_type);
            }
            '<' => {
                let t_type = if self.match_next('=') {
                    LessEqual
                } else {
                    Less
                };
                self.add_token(t_type);
            }
            '>' => {
                let t_type = if self.match_next('=') {
                    GreaterEqual
                } else {
                    Greater
                };
                self.add_token(t_type);
            }
            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else if self.match_next('*') {
                    self.block_comment();
                } else {
                    self.add_token(Slash);
                }
            }
            ' ' => {}
            '\r' => {} // Whitespace characters restart the loop
            '\t' => {}
            '\n' => {
                self.line += 1;
                self.col = 0;
            }
            '"' => self.string(),
            c => {
                // We can make use of u8: is_ascii_digit() and is_ascii_alphanumeric()
                if c.is_digit(10) {
                    self.number();
                // If a lexeme begins with a letter or underscore, we can assume its an identifier
                } else if c.is_alphabetic() || c == '_' {
                    self.identifier();
                } else {
                    self.rulox.error_line(
                        self.line,
                        format!("Unexpected character: {:?} = {:?}", c, c as char),
                    );
                }
            }
        }
    }

    // Advancing consists of two parts:
    // 1. advancing the current cursor one ahead
    // 2. returning (eating) the unadvanced character
    fn advance(&mut self) -> char {
        let current_char = self.source[self.current];
        self.current += 1;
        self.col += 1;
        current_char
    }

    // At the moment adding a token consists of pushing a Token with the proper type and the line #
    fn add_token(&mut self, t_type: TokenType) {
        self.tokens.push(Token {
            t_type,
            line: self.line,
            col: self.col,
        });
    }

    // Check to see if the next character belongs to the two-character token types
    // If so, advance the current cursor
    fn match_next(&mut self, expected_char: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source[self.current] != expected_char {
            return false;
        }

        self.current += 1;
        self.col += 1;
        true
    }

    // For longer lexemes, once we detect the beginning, we keep eating characters until the end
    // Peek looks at the current byte without consuming it
    // We end up with one character of look-ahead
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source[self.current]
    }

    // Look ahead two bytes without consuming the current character
    fn peek_second(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        self.source[self.current + 1]
    }

    // Additional element: C-style multi-line block comments, i.e. /* .... */
    // Structure similar to string() below, except we look ahead twice to check end
    // of block comment
    // Possible modifications: preserve comments, rework advance() to accept arguments
    // that determine number of characters to advance, although perhaps over-engineering
    fn block_comment(&mut self) {
        while self.peek() != '*' && self.peek_second() != '/' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
                self.col = 0;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.rulox
                .error_line(self.line, "Unterminated block comment.".to_string());
            return;
        }
        // We have reached the closing */, advance the current cursor ahead
        // Since it's a comment, for the moment we're simply consuming the bytes
        // and not producing any tokens.
        self.advance();
        self.advance();
    }

    // Upon consuming " advance current cursor until closing " or EOF
    // Since self.start remains at the opening " we can use it to consume the entire string
    // .advance() also returns a byte but we are not interested in it here
    // Could probably split functionality of "advance cursor" and "consume byte"
    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
                self.col = 0;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.rulox
                .error_line(self.line, "Unterminated string.".to_string());
            return;
        }
        // We have reached the close "", advance the current cursor ahead
        self.advance();

        // Trim surrounding quotes
        // Need a reference to the source, otherwise it doesn't know the correct size
        let value: String = self.source[self.start + 1..self.current - 1]
            .iter()
            .collect();
        self.add_token(StringLit(value));
    }

    // Like with string(), once we know we're in a number, we consume the full literal
    fn number(&mut self) {
        while self.peek().is_ascii_digit() {
            self.advance();
        }

        // Look for a fractional part
        if self.peek() == '.' && self.peek_second().is_ascii_digit() {
            // Consume the '.'
            self.advance();

            while self.peek().is_ascii_digit() {
                self.advance();
            }
        }

        // Span from the start of the literal to the current cursor
        // With Rust we need to unwrap both constructing a &str from bytes
        // And parsing that &str as a number
        let num_string: String = self.source[self.start..self.current].iter().collect();
        self.add_token(NumLit(num_string.parse().unwrap()));
    }

    // After strings and numbers, the remaining case is alphanumeric identifiers
    // As long as we have an uninterrupted string, capture the whole and check first
    // for a list of reserved keywords.
    // If the string is not a keyword, we have a user defined identifier literal
    fn identifier(&mut self) {
        while self.peek().is_alphanumeric() || self.peek() == '_' {
            self.advance();
        }

        let text: String = self.source[self.start..self.current].iter().collect();
        let t_type = match text.as_str() {
            "λ" => TokenType::Fun,
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            name => Identifier(name.to_owned()),
        };
        self.add_token(t_type);
    }
}
