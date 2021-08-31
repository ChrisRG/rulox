use super::ast::{Expr, Stmt};
use super::function::FunctionType;
use super::interpreter::Interpreter;
use std::collections::HashMap;
use std::mem;
use std::rc::Rc;

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<Rc<str>, bool>>,
    current_function: FunctionType,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Self {
            interpreter,
            scopes: Vec::new(),
            current_function: FunctionType::None,
        }
    }

    pub fn resolve_source(&mut self, stmts: &[Stmt]) -> Result<(), ()> {
        for stmt in stmts {
            self.resolve_stmt(stmt.clone());
        }
        Ok(())
    }

    fn resolve_stmt(&mut self, stmt: Stmt) {
        match stmt {
            Stmt::Expression(expr) | Stmt::Print(expr) => self.resolve_expr(expr),
            Stmt::Block(statements) => {
                self.begin_scope();
                for stmt in statements {
                    self.resolve_stmt(stmt);
                }
                self.end_scope();
            }
            Stmt::Var { name, initializer } => {
                self.declare(Rc::clone(&name));
                if let Some(initializer) = initializer {
                    self.resolve_expr(initializer);
                }
                self.define(Rc::clone(&name));
            }
            Stmt::Function {
                name,
                parameters,
                body,
            } => {
                self.declare(Rc::clone(&name));
                self.define(Rc::clone(&name));
                self.resolve_function(parameters, body, FunctionType::Function);
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                self.resolve_expr(condition);
                self.resolve_stmt(*then_branch);
                if let Some(e_branch) = else_branch {
                    self.resolve_stmt(*e_branch);
                }
            }
            Stmt::While { condition, body } => {
                self.resolve_expr(condition);
                self.resolve_stmt(*body);
            }
            Stmt::Return(expr) => {
                if let FunctionType::None = self.current_function {
                    eprintln!("Can't return from top-level code.");
                }

                self.resolve_expr(expr);
            }
            Stmt::Class { name, .. } => {
                self.declare(Rc::clone(&name));
                self.define(Rc::clone(&name));
            }
        }
    }

    fn resolve_expr(&mut self, expr: Expr) {
        match expr {
            Expr::Variable(name) => {
                if let Some(scope) = self.scopes.last() {
                    if let Some(initialized) = scope.get(&name) {
                        if *initialized == false {
                            eprintln!("Can't read local variable in its own initializer.");
                        }
                    }
                }
                self.resolve_local(name);
            }
            Expr::Assign { name, value } => {
                self.resolve_expr(*value);
                self.resolve_local(name);
            }
            Expr::Binary {
                lh_expr, rh_expr, ..
            } => {
                self.resolve_expr(*lh_expr);
                self.resolve_expr(*rh_expr);
            }
            Expr::Unary { rh_expr, .. } => {
                self.resolve_expr(*rh_expr);
            }
            Expr::Call {
                callee, arguments, ..
            } => {
                self.resolve_expr(*callee);
                for arg in arguments {
                    self.resolve_expr(arg);
                }
            }
            Expr::Literal(_) => {}
            Expr::Logical {
                lh_expr, rh_expr, ..
            } => {
                self.resolve_expr(*lh_expr);
                self.resolve_expr(*rh_expr);
            }
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: Rc<str>) {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.insert(Rc::clone(&name), false).is_some() {
                eprintln!("Variable with this name already declared in this scope.");
                return;
            }
        }
    }

    fn define(&mut self, name: Rc<str>) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(Rc::clone(&name), true);
        }
    }

    fn resolve_local(&mut self, name: Rc<str>) {
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&name) {
                self.interpreter.resolve(Rc::clone(&name), i);
            }
        }
    }

    fn resolve_function(
        &mut self,
        parameters: Vec<Rc<str>>,
        body: Vec<Stmt>,
        f_type: FunctionType,
    ) {
        let enclosing_function = mem::replace(&mut self.current_function, f_type);

        self.begin_scope();
        for param in parameters {
            self.declare(Rc::clone(&param));
            self.define(Rc::clone(&param));
        }
        self.resolve_stmt(Stmt::Block(body));
        self.end_scope();
        self.current_function = enclosing_function;
    }
}
