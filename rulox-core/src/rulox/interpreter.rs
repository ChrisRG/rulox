// use super::ast;
use crate::rulox::ast::Expr::*;
use crate::rulox::ast::{BiOperator, Expr, LogicOperator, Stmt, UnOperator, Value};
use crate::rulox::environment::Environment;
use crate::rulox::function::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
pub type EnvCell = Rc<RefCell<Environment>>;

pub struct Interpreter {
    pub globals: EnvCell,
    pub environment: EnvCell,
    pub locals: HashMap<Rc<str>, usize>,
    pub output: Vec<String>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let globals = Rc::new(RefCell::new(Environment::new(None)));
        // Basic clock to add to globals
        // globals
        //     .borrow_mut()
        //     .define(Rc::clone(&Clock.name()), Value::Callable(Rc::new(Clock)));
        let environment = Rc::clone(&globals);
        Interpreter {
            globals,
            environment,
            locals: HashMap::new(),
            output: Vec::new(),
        }
    }

    pub fn get_environment(&self) -> String {
        let env_string = format!("Environment: {}", self.environment.borrow());
        let envs = vec![env_string];

        envs.join("\n")
    }

    pub fn interpret(&mut self, statements: Vec<Stmt>) -> Vec<String> {
        for statement in statements {
            match self.execute(&statement) {
                Ok(_) => {}
                Err(e) => {
                    self.add_print_result(String::from("Runtime error."));
                    let msg = format!("{:?}", e);
                    self.add_print_result(msg);
                    break;
                }
            };
        }
        self.output.clone()
    }

    // Since the original requires passing around Java Objects, we will continue
    // to use enums to pass around expressions and literals, this requires some
    // rather bloated handling of the enums, such as unwrapping and re-wrapping
    // to guarantee consistent types. The output will be a value of some type:
    // String(Box<String>), Number(f64), Boolean(bool), or Nil.
    pub fn evaluate(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
        let result = match expr {
            // Unwrap a literal and return a copy of its value
            Literal(val) => val.clone(),
            Unary { op, rh_expr } => {
                // Recursively evalute rh_expr if necessary
                let right = &self.evaluate(rh_expr)?;

                match op {
                    // Extracting the number and negating it leaves us with f64, need to re-wrap it
                    UnOperator::Minus => Value::Number(-extract_number(right)?),
                    // Get the truth value of the rh_expr and return the negation
                    UnOperator::Bang => Value::Boolean(!(is_truthy(right))),
                }
            }
            Binary {
                lh_expr,
                op,
                rh_expr,
            } => {
                let left = &self.evaluate(lh_expr)?;
                let right = &self.evaluate(rh_expr)?;
                match op {
                    BiOperator::Minus => {
                        Value::Number(extract_number(left)? - extract_number(right)?)
                    }
                    BiOperator::Slash => {
                        Value::Number(extract_number(left)? / extract_number(right)?)
                    }
                    BiOperator::Star => {
                        Value::Number(extract_number(left)? * extract_number(right)?)
                    }
                    BiOperator::Plus => match (left, right) {
                        (Value::Number(left), Value::Number(right)) => Value::Number(left + right),
                        (Value::String(left), Value::String(right)) => {
                            Value::String(Box::new(format!("{}{}", &left, &right)))
                        }
                        (Value::Number(_), right) => {
                            return Err(RuntimeError::type_error(right, "Expected number"))
                        }
                        (Value::String(_), right) => {
                            return Err(RuntimeError::type_error(right, "Expected string"))
                        }
                        (left, _) => {
                            return Err(RuntimeError::type_error(left, "Expected number or string"))
                        }
                    },
                    BiOperator::Greater => {
                        Value::Boolean(extract_number(left)? > extract_number(right)?)
                    }
                    BiOperator::GreaterEqual => {
                        Value::Boolean(extract_number(left)? >= extract_number(right)?)
                    }
                    BiOperator::Less => {
                        Value::Boolean(extract_number(left)? < extract_number(right)?)
                    }
                    BiOperator::LessEqual => {
                        Value::Boolean(extract_number(left)? <= extract_number(right)?)
                    }
                    BiOperator::Eq => Value::Boolean(left == right),
                    BiOperator::NotEq => Value::Boolean(left != right),
                }
            }
            Variable(name) => {
                self.environment.borrow().get(name)?
                // self.look_up_var(name.to_owned())?
            }

            Assign { name, value } => {
                let value = self.evaluate(value)?;
                if let Some(distance) = self.locals.get(name) {
                    self.environment.borrow_mut().assign_at(
                        distance,
                        Rc::clone(name),
                        value.clone(),
                    )?;
                } else {
                    self.environment
                        .borrow_mut()
                        .assign(Rc::clone(name), value.clone())?;
                }
                value
            }
            Logical {
                lh_expr,
                op,
                rh_expr,
            } => {
                let left = self.evaluate(lh_expr)?;
                match op {
                    LogicOperator::Or if is_truthy(&left) => left,
                    LogicOperator::And if !is_truthy(&left) => left,
                    _ => self.evaluate(rh_expr)?,
                }
            }
            // TODO: include paren in parameters to pass line number to error
            Call {
                callee, arguments, ..
            } => {
                let callee = self.evaluate(callee)?;
                let mut args = Vec::with_capacity(arguments.len());
                for arg_expr in arguments {
                    args.push(self.evaluate(arg_expr)?);
                }
                if let Some(function) = callee.into_callable() {
                    let arity = function.arity();
                    if arguments.len() != arity {
                        return Err(RuntimeError::ArityError {
                            expected: arity,
                            got: arguments.len(),
                        });
                    }
                    function.call(self, args)?
                } else {
                    return Err(RuntimeError::CallableError {
                        msg: "Can only call functions and classes",
                    });
                }
            }
        };
        Ok(result)
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        match stmt {
            // TODO: For Print need to handle errors, since cannot use Display for Result
            Stmt::Print(expr) => {
                let msg = format!("{}", self.evaluate(expr)?);
                self.add_print_result(msg);
            }
            Stmt::Expression(expr) => {
                self.evaluate(expr)?;
            }
            Stmt::Var { name, initializer } => {
                let value = match initializer {
                    Some(expr) => self.evaluate(&expr)?,
                    None => Value::Nil,
                };
                self.environment.borrow_mut().define(name.to_owned(), value);
            }
            Stmt::Block(statements) => {
                let new_env = Environment::new(Some(Rc::clone(&self.environment))).into_cell();
                self.execute_block(statements, new_env)?
            }
            Stmt::If {
                condition,
                then_branch,
                else_branch,
            } => {
                if is_truthy(&self.evaluate(condition)?) {
                    self.execute(then_branch)?;
                } else if let Some(else_branch) = else_branch {
                    self.execute(else_branch)?;
                }
            }
            Stmt::While { condition, body } => {
                while is_truthy(&self.evaluate(condition)?) {
                    self.execute(body)?;
                }
            }
            Stmt::Function {
                name,
                parameters,
                body,
            } => {
                let function = LoxFunction::new(
                    name.clone(),
                    parameters.clone(),
                    body.clone(),
                    Rc::clone(&self.environment),
                );
                self.environment
                    .borrow_mut()
                    .define(Rc::clone(name), Value::Callable(Rc::new(function)));
            }
            Stmt::Return(expr) => {
                let val = self.evaluate(expr)?;

                return Err(RuntimeError::Return(val));
            }
            // Stmt::Class { name, methods } => {
            //     self.environment
            //         .borrow_mut()
            //         .define(Rc::clone(&name), Value::Nil);
            //     let class = LoxClass::new(Rc::clone(&name));
            //     self.environment
            //         .borrow_mut()
            //         .assign(Rc::clone(&name), Value::Callable(Rc::new(class)));
            // }
            _ => unreachable!(),
        }
        Ok(())
    }

    pub fn execute_block(
        &mut self,
        statements: &[Stmt],
        new_env: EnvCell,
    ) -> Result<(), RuntimeError> {
        let prev_env = self.environment.clone();

        let mut result = Ok(());

        self.environment = new_env;
        for stmt in statements {
            result = self.execute(stmt);

            if result.is_err() {
                break;
            }
        }

        self.environment = prev_env;
        result
    }

    pub fn resolve(&mut self, name: Rc<str>, depth: usize) {
        self.locals.insert(name, depth);
    }

    // fn look_up_var(&mut self, name: Rc<str>) -> Result<Value, RuntimeError> {
    //     let distance = self.locals.get(&name);
    //     let val = match distance {
    //         Some(depth) => self.environment.borrow().get_at(distance.unwrap(), &name),
    //         None => self.globals.borrow().get(&name),
    //     };
    //     val
    // }

    fn add_print_result(&mut self, msg: String) {
        self.output.push(msg);
    }
}

fn is_truthy(val: &Value) -> bool {
    match val {
        Value::Boolean(false) | Value::Nil => false,
        _ => true,
    }
}

// Checks whether a Value holds an actual number, which it returns unwrapped
// Otherwise, throws a runtime error
fn extract_number(val: &Value) -> Result<f64, RuntimeError> {
    match val {
        Value::Number(num) => Ok(*num),
        _ => Err(RuntimeError::type_error(val, "Operand must be a number.")),
    }
}

#[derive(Debug, Clone)]
pub enum RuntimeError {
    TypeError { val: String, msg: &'static str },
    UndefinedError { name: String },
    CallableError { msg: &'static str },
    ArityError { expected: usize, got: usize },
    Return(Value),
}

impl RuntimeError {
    fn type_error(val: &Value, msg: &'static str) -> RuntimeError {
        RuntimeError::TypeError {
            val: val.to_string(),
            msg,
        }
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RuntimeError::TypeError { val, msg } => write!(f, "Found {}; {}", val, msg),
            RuntimeError::UndefinedError { name } => write!(f, "Undefined variable '{}'", name),
            RuntimeError::CallableError { msg } => write!(f, "{}", msg),
            RuntimeError::ArityError { expected, got } => {
                write!(f, "Expected {} arguments but got {}", expected, got)
            }
            RuntimeError::Return { .. } => unreachable!(),
        }
    }
}
