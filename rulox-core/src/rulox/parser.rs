// Expression grammar:
//
// expression     → equality ;
// equality       → comparison ( ( "!=" | "==" ) comparison )* ;
// comparison     → term ( ( ">" | ">=" | "<" | "<=" ) term )* ;
// term           → factor ( ( "-" | "+" ) factor )* ;
// factor         → unary ( ( "/" | "*" ) unary )* ;
// unary          → ( "!" | "-" ) unary
//                | primary ;
// primary        → NUMBER | STRING | "true" | "false" | "nil"
//                | "(" expression ")" ;
//
// Translating to code:
// Grammar notation 	  Code representation
//      Terminal	          Code to match and consume a token
//      Nonterminal	        Call to that rule’s function
//      |	                  if or switch statement
//      * or +	            while or for loop
//      ?	                  if statement
//
// Type of parsing: Recursive descent

#![allow(dead_code)]
#[allow(clippy::all)]
use std::fmt;
use std::rc::Rc;

use super::ast::{BiOperator, Expr, LogicOperator, Stmt, UnOperator, Value};
use super::token::TokenType::*;
use super::token::{Token, TokenType};
use super::Rulox;

#[derive(Debug, Clone)]
struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parsing error")
    }
}

pub struct Parser<'a> {
    tokens: Vec<Token>,
    current: usize,
    rulox: &'a mut Rulox,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>, rulox: &'a mut Rulox) -> Parser<'a> {
        Parser {
            tokens,
            rulox,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Vec<Stmt> {
        let mut statements = Vec::new();
        while !self.is_at_end() {
            match self.declaration() {
                Ok(stmt) => statements.push(stmt),
                Err(_) => statements.push(Stmt::Return(Expr::Literal(Value::Nil))),
            }
        }
        statements
    }

    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.or()?;

        if self.check(vec![Equal]).is_some() {
            let value = self.assignment()?;

            match expr {
                Expr::Variable(name) => Ok(Expr::Assign {
                    name,
                    value: Box::new(value),
                }),
                _ => Err(ParseError),
            }
        } else {
            Ok(expr)
        }
    }

    fn or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.and()?;

        while self.check(vec![Or]).is_some() {
            let right = self.and()?;
            expr = Expr::Logical {
                lh_expr: Box::new(expr),
                op: LogicOperator::Or,
                rh_expr: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;

        while self.check(vec![And]).is_some() {
            let right = self.equality()?;
            expr = Expr::Logical {
                lh_expr: Box::new(expr),
                op: LogicOperator::And,
                rh_expr: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;

        while let Some(op_token) = self.check(vec![BangEqual, EqualEqual]) {
            let op: BiOperator = if op_token == BangEqual {
                BiOperator::NotEq
            } else {
                BiOperator::Eq
            };
            let right: Expr = self.comparison()?;
            expr = Expr::Binary {
                lh_expr: Box::new(expr),
                op,
                rh_expr: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;

        while let Some(op_token) = self.check(vec![Greater, GreaterEqual, Less, LessEqual]) {
            let op: BiOperator = match op_token {
                Greater => BiOperator::Greater,
                GreaterEqual => BiOperator::GreaterEqual,
                Less => BiOperator::Less,
                LessEqual => BiOperator::LessEqual,
                _ => unreachable!(),
            };
            let right: Expr = self.term()?;
            expr = Expr::Binary {
                lh_expr: Box::new(expr),
                op,
                rh_expr: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        while let Some(op_token) = self.check(vec![Minus, Plus]) {
            let op: BiOperator = if op_token == Minus {
                BiOperator::Minus
            } else {
                BiOperator::Plus
            };
            let right: Expr = self.factor()?;
            expr = Expr::Binary {
                lh_expr: Box::new(expr),
                op,
                rh_expr: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while let Some(op_token) = self.check(vec![Slash, Star]) {
            let op: BiOperator = if op_token == Slash {
                BiOperator::Slash
            } else {
                BiOperator::Star
            };
            let right: Expr = self.unary()?;
            expr = Expr::Binary {
                lh_expr: Box::new(expr),
                op,
                rh_expr: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParseError> {
        if let Some(op_token) = self.check(vec![Bang, Minus]) {
            let op = if op_token == Bang {
                UnOperator::Bang
            } else {
                UnOperator::Minus
            };
            let right: Expr = self.unary()?;
            return Ok(Expr::Unary {
                op,
                rh_expr: Box::new(right),
            });
        }
        self.call()
    }

    fn call(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.primary()?;

        while self.check(vec![LeftParen]).is_some() {
            expr = self.finish_call(expr)?;
        }

        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, ParseError> {
        let mut arguments = Vec::new();

        if self.check(vec![RightParen]).is_none() {
            loop {
                if arguments.len() > 8 {
                    self.error(String::from("Can't have more than 8 arguments."));
                }
                arguments.push(self.expression()?);
                if self.check(vec![Comma]).is_none() {
                    break;
                }
            }
            self.consume(RightParen, String::from("Expect ')' after arguments"));
        }
        Ok(Expr::Call {
            callee: Box::new(callee),
            paren: RightParen,
            arguments,
        })
    }

    fn primary(&mut self) -> Result<Expr, ParseError> {
        let expr = match &self.advance().t_type {
            False => Expr::Literal(Value::Boolean(false)),
            True => Expr::Literal(Value::Boolean(true)),
            Nil => Expr::Literal(Value::Nil),
            NumLit(num) => Expr::Literal(Value::Number(*num)),
            StringLit(s) => Expr::Literal(Value::String(Box::new(s.clone()))),
            Identifier(name) => Expr::Variable(Rc::from(name.to_owned())),
            LeftParen => {
                let expr = self.expression()?;
                self.consume(
                    RightParen,
                    "Expect ')' after grouped expression.".to_string(),
                );
                expr
            }
            _ => return Err(self.error("Expect expression.".to_string())),
        };

        Ok(expr)
    }

    // declaration -> fun_declaration | var_declaration | statement
    fn declaration(&mut self) -> Result<Stmt, ParseError> {
        match self.check(vec![Var, Fun]) {
            // Some(Class) => self.class_declaration(),
            Some(Var) => self.var_declaration(),
            Some(Fun) => self.function("function"),
            _ => self.statement(),
        }
    }

    fn statement(&mut self) -> Result<Stmt, ParseError> {
        match self.check(vec![If, Print, LeftBrace, While, For, Return]) {
            Some(For) => self.for_statement(),
            Some(If) => self.if_statement(),
            Some(Print) => self.print_stmt(),
            Some(While) => self.while_statement(),
            Some(LeftBrace) => Ok(Stmt::Block(self.block()?)),
            Some(Return) => self.return_statement(),
            _ => self.expr_stmt(),
        }
    }

    fn class_declaration(&mut self) -> Result<Stmt, ParseError> {
        // Class has already been consumed in declaration via self.check()
        let name = Rc::from(self.consume_identifier(String::from("Expect class name."))?);
        self.consume(LeftBrace, String::from("Expect '{' before class body."));

        let mut methods = Vec::new();
        while self.peek().t_type != RightBrace && self.peek().t_type != Eof {
            // while (!self.check(vec![RightBrace]).is_some() && !self.is_at_end()) {
            methods.push(self.function("method")?);
        }

        self.consume(RightBrace, String::from("Expect '}' after class body."));

        Ok(Stmt::Class {
            name: Rc::clone(&name),
            methods,
        })
    }

    fn var_declaration(&mut self) -> Result<Stmt, ParseError> {
        // Var has already been consumed in declaration via self.check()
        let name = Rc::from(self.consume_identifier(String::from("Expect variable name."))?);
        let initializer = match self.check(vec![Equal]) {
            Some(_) => Some(self.expression()?),
            // None => Expr::Literal(Value::Nil),
            None => None,
        };
        self.consume(
            Semicolon,
            String::from("Expect ';' after variable declaration."),
        );
        Ok(Stmt::Var {
            name,
            initializer: initializer,
        })
    }

    fn function(&mut self, kind: &'static str) -> Result<Stmt, ParseError> {
        let name = Rc::from(self.consume_identifier(format!("Expect {} name", kind))?);
        self.consume(LeftParen, format!("Expect '(' after {} name", kind));
        let mut parameters = Vec::new();
        if self.check(vec![RightParen]).is_none() {
            loop {
                if parameters.len() > 8 {
                    self.error(format!("Can't have more than 8 parameters"));
                }
                parameters.push(Rc::from(
                    self.consume_identifier(format!("Expect parameter name."))?,
                ));
                if self.check(vec![Comma]).is_none() {
                    break;
                }
            }

            self.consume(RightParen, format!("Expect ')' after parameters."));
        }

        self.consume(LeftBrace, format!("Expect {{ before {} body.", kind));

        let body = self.block()?;

        Ok(Stmt::Function {
            name,
            parameters,
            body,
        })
    }

    fn expr_stmt(&mut self) -> Result<Stmt, ParseError> {
        let expr: Expr = self.expression()?;
        self.consume(Semicolon, String::from("Expect ';' after expression."));
        Ok(Stmt::Expression(expr))
    }

    fn print_stmt(&mut self) -> Result<Stmt, ParseError> {
        let value: Expr = self.expression()?;
        self.consume(Semicolon, String::from("Expect ';' after value."));
        Ok(Stmt::Print(value))
    }

    fn block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        let mut statements = Vec::new();

        while self.peek().t_type != RightBrace && self.peek().t_type != Eof {
            statements.push(self.declaration()?);
        }

        self.consume(RightBrace, String::from("Expect '}' after block."));
        Ok(statements)
    }

    fn if_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(LeftParen, String::from("Expect '(' after 'if'."));
        let condition = self.expression()?;
        self.consume(RightParen, String::from("Expect ')' after condition."));

        let then_branch = Box::new(self.statement()?);
        let else_branch = if self.check(vec![Else]).is_some() {
            Some(self.statement()?)
        } else {
            None
        }
        .map(Box::new);

        Ok(Stmt::If {
            condition,
            then_branch,
            else_branch,
        })
    }

    fn while_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(LeftParen, String::from("Expect '(' after 'while'."));
        let condition = self.expression()?;
        self.consume(
            RightParen,
            String::from("Expect ')' after while condition."),
        );

        let body = self.statement()?;

        Ok(Stmt::While {
            condition,
            body: Box::new(body),
        })
    }

    fn for_statement(&mut self) -> Result<Stmt, ParseError> {
        self.consume(LeftParen, String::from("Expect '(' after 'for'."));

        let initializer = match self.check(vec![Semicolon, Var]) {
            Some(Semicolon) => None,
            Some(Var) => Some(self.var_declaration()?),
            _ => Some(self.expr_stmt()?),
        };

        let condition = if self.peek().t_type == Semicolon {
            Expr::Literal(Value::Boolean(true))
        } else {
            self.expression()?
        };

        self.consume(
            Semicolon,
            String::from("Expect ';' after for loop condition."),
        );

        let increment = if self.peek().t_type == RightParen {
            None
        } else {
            Some(self.expression()?)
        };

        self.consume(RightParen, String::from("Expect ')' after for clauses."));

        let mut body = self.statement()?;

        if let Some(increment) = increment {
            body = Stmt::Block(vec![body, Stmt::Expression(increment)]);
        }

        body = Stmt::While {
            condition,
            body: Box::new(body),
        };

        if let Some(initializer) = initializer {
            body = Stmt::Block(vec![initializer, body]);
        }

        Ok(body)
    }

    fn return_statement(&mut self) -> Result<Stmt, ParseError> {
        let val = if self.check(vec![Semicolon]).is_none() {
            self.expression()?
        } else {
            Expr::Literal(Value::Nil)
        };

        self.consume(Semicolon, format!("Expect ';' after return value."));
        Ok(Stmt::Return(val))
    }

    fn check(&mut self, expected_types: Vec<TokenType>) -> Option<TokenType> {
        for t_type in expected_types.iter() {
            if self.check_type(t_type) {
                return Some(self.advance().t_type.clone());
            }
        }
        None
    }

    fn check_type(&mut self, t_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        &self.peek().t_type == t_type
    }

    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        &self.previous()
    }

    fn is_at_end(&mut self) -> bool {
        self.peek().t_type == Eof
    }

    fn peek(&mut self) -> &Token {
        &self.tokens[self.current]
    }

    fn previous(&mut self) -> &Token {
        // Poor handling of underflow, need to rewrite
        // TODO: eliminate previous and just do some simple check + lookahead on tokens
        if self.current > 0 {
            &self.tokens[self.current - 1]
        } else {
            &self.tokens[0]
        }
    }

    fn consume(&mut self, t_type: TokenType, msg: String) {
        if self.check_type(&t_type) {
            self.advance();
        } else {
            self.error(msg);
        }
    }

    fn consume_identifier(&mut self, msg: String) -> Result<String, ParseError> {
        match self.peek().t_type {
            Identifier(_) => match &self.advance().t_type {
                Identifier(s) => Ok(s.to_owned()),
                _ => unreachable!(),
            },
            _ => Err(self.error(msg)),
        }
    }

    fn error(&mut self, msg: String) -> ParseError {
        // Poor handling of underflow, need to rewrite
        if self.current > 0 {
            let current_token = &self.tokens[self.current - 1];
            self.rulox.error_token(
                current_token.t_type.to_string(),
                current_token.line,
                current_token.col,
                msg,
            );
        } else {
            let current_token = &self.tokens[0];
            self.rulox
                .error_token(current_token.t_type.to_string(), 1, current_token.col, msg);
        }
        ParseError
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            // if self.previous().t_type == Semicolon { return; }

            if [Eof, Class, Fun, Var, For, If, While, Print, Return].contains(&self.peek().t_type) {
                break;
            }

            self.advance();
        }
    }
}
