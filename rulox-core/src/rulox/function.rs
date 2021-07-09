use super::ast::{Stmt, Value};
use super::environment::Environment;
use super::interpreter::{Interpreter, RuntimeError};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;
// use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Copy, PartialEq)]
pub enum FunctionType {
    None,
    Function,
}

// pub trait LoxCallable: fmt::Debug {
//     fn call(&self, interpreter: &mut Interpreter, args: Vec<Value>) -> Result<Value, RuntimeError>;
//     fn arity(&self) -> usize;
//     fn name(&self) -> Rc<str>;
//     fn parameters(&self) -> Vec<Rc<str>>;
//     fn body(&self) -> Vec<Stmt>;
// }

// impl fmt::Display for dyn LoxCallable {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "<fn {} ({:?})>", self.name(), self.parameters())
//     }
// }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoxFunction {
    name: Rc<str>,
    closure: Rc<RefCell<Environment>>,
    parameters: Vec<Rc<str>>,
    body: Vec<Stmt>,
}

impl LoxFunction {
    pub fn new(
        name: Rc<str>,
        parameters: Vec<Rc<str>>,
        body: Vec<Stmt>,
        closure: Rc<RefCell<Environment>>,
    ) -> Self {
        Self {
            name,
            closure,
            parameters,
            body,
        }
    }
    pub fn call(
        &self,
        interpreter: &mut Interpreter,
        args: Vec<Value>,
    ) -> Result<Value, RuntimeError> {
        let mut env = Environment::new(Some(Rc::clone(&self.closure)));
        for (param, argument) in self.parameters.iter().zip(args.iter()) {
            env.define(param.to_owned(), argument.clone());
        }

        let result = interpreter.execute_block(&self.body, env.into_cell());

        match result {
            Ok(()) => Ok(Value::Nil),
            Err(RuntimeError::Return(val)) => Ok(val),
            Err(e) => Err(e),
        }
    }

    pub fn arity(&self) -> usize {
        self.parameters.len()
    }
}

impl fmt::Display for LoxFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(fn {} ({}))", self.name, self.parameters.join(", "))
    }
}

// #[derive(Debug, Serialize, Deserialize)]
// pub struct LoxClass {
//     name: Rc<str>,
// }

// impl LoxClass {
//     pub fn new(name: Rc<str>) -> Self {
//         Self { name }
//     }
//     fn call(
//         &self,
//         _interpreter: &mut Interpreter,
//         _args: Vec<Value>,
//     ) -> Result<Value, RuntimeError> {
//         Ok(Value::Nil)
//     }
//     fn arity(&self) -> usize {
//         0
//     }
//     fn name(&self) -> Rc<str> {
//         Rc::clone(&self.name)
//     }

//     fn body(&self) -> Vec<Stmt> {
//         vec![Stmt::Expression(Expr::Literal(Value::Nil))]
//     }

//     fn parameters(&self) -> Vec<Rc<str>> {
//         vec![Rc::clone(&self.name())]
//     }
// }

// impl fmt::Display for LoxClass {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "<class {}>", self.name)
//     }
// }

// ------------------------------
// ---- Lox Standard Library ----
// ------------------------------

// #[derive(Debug)]
// pub struct Clock;

// impl LoxCallable for Clock {
//     fn call(
//         &self,
//         _interpreter: &mut Interpreter,
//         _args: Vec<Value>,
//     ) -> Result<Value, RuntimeError> {
//         Ok(Value::Number(
//             SystemTime::now()
//                 .duration_since(UNIX_EPOCH)
//                 .expect("Could not retrieve time.")
//                 .as_millis() as f64,
//         ))
//     }

//     fn arity(&self) -> usize {
//         0
//     }

//     fn name(&self) -> Rc<str> {
//         Rc::from("clock")
//     }
// }
