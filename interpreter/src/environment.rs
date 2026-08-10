//! Лексические окружения (lexical environments).
//!
//! Окружение хранит переменные и ссылку на родительское окружение.
//! Это основа для реализации замыканий и области видимости.

use crate::value::Value;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

/// Окружение (environment).
#[derive(Debug, Clone, Default)]
pub struct Environment {
    /// Переменные, объявленные в этом окружении.
    values: HashMap<String, Value>,
    /// Родительское окружение.
    parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    /// Создаёт новое глобальное окружение.
    pub fn new() -> Self {
        Environment { values: HashMap::new(), parent: None }
    }

    /// Создаёт дочернее окружение с родителем.
    pub fn with_parent(parent: Rc<RefCell<Environment>>) -> Self {
        Environment { values: HashMap::new(), parent: Some(parent) }
    }

    /// Определяет переменную в текущем окружении.
    pub fn define(&mut self, name: &str, value: Value) {
        self.values.insert(name.to_string(), value);
    }

    /// Получает значение переменной, проходя по цепочке окружений.
    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(value) = self.values.get(name) {
            return Some(value.clone());
        }
        if let Some(parent) = &self.parent {
            return parent.borrow().get(name);
        }
        None
    }

    /// Присваивает значение переменной, проходя по цепочке окружений.
    pub fn assign(&mut self, name: &str, value: Value) -> bool {
        if self.values.contains_key(name) {
            self.values.insert(name.to_string(), value);
            return true;
        }
        if let Some(parent) = &self.parent {
            return parent.borrow_mut().assign(name, value);
        }
        false
    }

    /// Проверяет, определена ли переменная в текущем окружении.
    pub fn contains(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_define_and_get() {
        let mut env = Environment::new();
        env.define("x", Value::Number(42.0));
        assert_eq!(env.get("x"), Some(Value::Number(42.0)));
        assert_eq!(env.get("y"), None);
    }

    #[test]
    fn test_parent_chain() {
        let parent = Rc::new(RefCell::new(Environment::new()));
        parent.borrow_mut().define("x", Value::Number(1.0));

        let mut child = Environment::with_parent(parent);
        child.define("y", Value::Number(2.0));

        assert_eq!(child.get("x"), Some(Value::Number(1.0)));
        assert_eq!(child.get("y"), Some(Value::Number(2.0)));
    }

    #[test]
    fn test_assign() {
        let mut env = Environment::new();
        env.define("x", Value::Number(1.0));
        assert!(env.assign("x", Value::Number(99.0)));
        assert_eq!(env.get("x"), Some(Value::Number(99.0)));
        assert!(!env.assign("missing", Value::Number(1.0)));
    }
}