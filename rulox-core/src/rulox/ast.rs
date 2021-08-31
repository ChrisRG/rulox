use serde::{Deserialize, Serialize};
use std::fmt;
use std::rc::Rc;

use super::function::LoxFunction;
use super::token::TokenType;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expr {
    Binary {
        lh_expr: Box<Expr>,
        op: BiOperator,
        rh_expr: Box<Expr>,
    },
    Unary {
        op: UnOperator,
        rh_expr: Box<Expr>,
    },
    Literal(Value),
    Assign {
        name: Rc<str>,
        value: Box<Expr>,
    },
    Variable(Rc<str>),
    Logical {
        lh_expr: Box<Expr>,
        op: LogicOperator,
        rh_expr: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: TokenType,
        arguments: Vec<Expr>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Stmt {
    Expression(Expr),
    Function {
        name: Rc<str>,
        parameters: Vec<Rc<str>>,
        body: Vec<Stmt>,
    },
    Print(Expr),
    Return(Expr),
    Var {
        name: Rc<str>,
        initializer: Option<Expr>,
    },
    Block(Vec<Stmt>),
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
    },
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    Class {
        name: Rc<str>,
        methods: Vec<Stmt>, // Assume that they're all Stmt::Function
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Value {
    String(Box<String>),
    Number(f64),
    Boolean(bool),
    Nil,
    Callable(Rc<LoxFunction>),
}

impl Value {
    pub fn into_callable(self) -> Option<Rc<LoxFunction>> {
        match self {
            Value::Callable(function) => Some(function),
            _ => None,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Value) -> bool {
        match (self, other) {
            (&Value::String(ref left), &Value::String(ref right)) => left == right,
            (&Value::Number(left), &Value::Number(right)) => left == right,
            (&Value::Boolean(left), &Value::Boolean(right)) => left == right,
            (&Value::Nil, &Value::Nil) => true,
            (&Value::Callable(ref left), &Value::Callable(ref right)) => Rc::ptr_eq(left, right),
            _ => false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BiOperator {
    Plus,
    Minus,
    Slash,
    Star,
    Eq,
    NotEq,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnOperator {
    Minus,
    Bang,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogicOperator {
    And,
    Or,
}

impl fmt::Display for BiOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BiOperator::Plus => write!(f, "+"),
            BiOperator::Minus => write!(f, "-"),
            BiOperator::Slash => write!(f, "/"),
            BiOperator::Star => write!(f, "*"),
            BiOperator::Eq => write!(f, "=="),
            BiOperator::NotEq => write!(f, "!="),
            BiOperator::Greater => write!(f, ">"),
            BiOperator::GreaterEqual => write!(f, ">="),
            BiOperator::Less => write!(f, "<"),
            BiOperator::LessEqual => write!(f, "<="),
        }
    }
}

impl fmt::Display for UnOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UnOperator::Bang => write!(f, "!"),
            UnOperator::Minus => write!(f, "-"),
        }
    }
}

impl fmt::Display for LogicOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogicOperator::And => write!(f, " and "),
            LogicOperator::Or => write!(f, " or "),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(val) => write!(f, "\"{}\"", val),
            Value::Number(num) => write!(f, "{}", num),
            Value::Boolean(b) => write!(f, "{}", b),
            Value::Nil => write!(f, "nil"),
            Value::Callable(ref callable) => write!(f, "{}", callable),
        }
    }
}
