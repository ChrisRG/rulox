use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use super::ast::Value;
use super::interpreter::RuntimeError;

pub type EnvCell = Rc<RefCell<Environment>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    values: HashMap<Rc<str>, Value>,
    enclosing: Option<EnvCell>,
}

impl Environment {
    pub fn new(parent: Option<EnvCell>) -> Environment {
        Environment {
            values: HashMap::new(),
            enclosing: parent,
        }
    }

    pub fn define(&mut self, name: Rc<str>, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Result<Value, RuntimeError> {
        // To pass a reference to Rc<name>, we need to borrow its dereferenced form -> &*name
        match self.values.get(&*name) {
            Some(val) => Ok(val.clone()),
            None => match &self.enclosing {
                Some(enclosing) => Ok(enclosing.borrow().get(name)?),
                None => Err(RuntimeError::UndefinedError {
                    name: name.to_owned(),
                }),
            },
        }
    }

    // Need to re-implement, used for checking scope depths
    // pub fn get_at(&self, distance: &usize, name: &str) -> Result<Value, RuntimeError> {
    //     if *distance > 0 {
    //         Ok(self
    //             .ancestor(distance)
    //             .unwrap()
    //             .borrow()
    //             .values
    //             .get(name)
    //             .expect(&format!("Undefined variable '{}'", name))
    //             .clone())
    //     } else {
    //         Ok(self
    //             .values
    //             .get(name)
    //             .expect(&format!("Undefined variable '{}'", name))
    //             .clone())
    //     }
    // }

    pub fn assign(&mut self, name: Rc<str>, value: Value) -> Result<(), RuntimeError> {
        if self.values.contains_key(&name) {
            self.values.insert(name, value);
            Ok(())
        } else {
            match &self.enclosing {
                Some(enclosing) => enclosing.borrow_mut().assign(name, value),
                None => Err(RuntimeError::UndefinedError {
                    name: name.to_string(),
                }),
            }
        }
    }

    pub fn assign_at(
        &mut self,
        distance: &usize,
        name: Rc<str>,
        value: Value,
    ) -> Result<(), RuntimeError> {
        if *distance > 0 {
            self.ancestor(distance)
                .unwrap()
                .borrow_mut()
                .values
                .insert(name, value);
        } else {
            self.values.insert(name, value);
        }
        Ok(())
    }

    fn ancestor(&self, distance: &usize) -> Option<EnvCell> {
        let parent = self.enclosing.clone()?;
        let mut environment = Rc::clone(&parent);
        for _ in 1..*distance {
            let parent = environment.borrow().enclosing.clone()?;
            environment = Rc::clone(&parent);
        }
        Some(environment)
    }

    pub fn into_cell(self) -> EnvCell {
        Rc::new(RefCell::new(self))
    }
}

impl fmt::Display for Environment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let output_values: Vec<String> = self
            .values
            .iter()
            .map(|(k, v)| format!("'{}':{}", k, v))
            .collect();
        write!(f, "{{ {} }}", output_values.join("; "))
    }
}
