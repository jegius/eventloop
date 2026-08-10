//! Представление значений JavaScript (`Value`).
//!
//! Значения JS моделируются через перечисление [`Value`]. Это базовый тип,
//! который используется интерпретатором для хранения и передачи данных.

use crate::ast::Stmt;
use std::cell::RefCell;
use std::rc::Rc;

/// Значение JavaScript.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// `undefined`
    Undefined,
    /// `null`
    Null,
    /// `true` / `false`
    Boolean(bool),
    /// Число (IEEE 754 double)
    Number(f64),
    /// Строка
    String(String),
    /// Объект (свойства)
    Object(Object),
    /// Массив
    Array(Vec<Value>),
    /// Функция (замыкание)
    Function(Function),
    /// Промис (хранится в Rc<RefCell> для поддержки цепочек `.then()`).
    Promise(Rc<RefCell<Promise>>),
}

/// Объект JavaScript: набор свойств.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Object {
    /// Свойства объекта (имя → значение).
    pub properties: std::collections::HashMap<String, Value>,
}

/// Функция JavaScript.
#[derive(Debug, Clone, PartialEq)]
pub struct Function {
    /// Имя функции (может быть пустым для анонимных).
    pub name: String,
    /// Параметры.
    pub params: Vec<String>,
    /// Тело функции (AST).
    pub body: Option<Box<Stmt>>,
    /// Является ли функция стрелочной.
    pub is_arrow: bool,
}

/// Промис.
#[derive(Debug, Clone, PartialEq)]
pub struct Promise {
    /// Состояние промиса.
    pub state: PromiseState,
    /// Значение (для fulfilled/rejected).
    pub value: Option<Box<Value>>,
    /// Колбэки `then`.
    pub then_callbacks: Vec<Value>,
    /// Колбэки `catch`.
    pub catch_callbacks: Vec<Value>,
    /// Является ли промис результатом `fetch` (сетевой запрос).
    /// Поле зарезервировано для визуализации; на поведение очередей не влияет
    /// (колбэки `.then` от fetch, как и от обычного промиса, попадают в микротаску).
    pub is_fetch: bool,
}

/// Состояние промиса.
#[derive(Debug, Clone, PartialEq)]
pub enum PromiseState {
    /// Ожидание.
    Pending,
    /// Выполнен.
    Fulfilled,
    /// Отклонён.
    Rejected,
}

impl Function {
    /// Возвращает строковое представление функции (для визуализации).
    pub fn display(&self) -> String {
        if let Some(body) = &self.body {
            format!("() => {}", body.display())
        } else {
            format!("function {}", self.name)
        }
    }
}

impl Value {
    /// Возвращает строковое представление значения (для визуализации колбэков).
    pub fn display(&self) -> String {
        match self {
            Value::Function(f) => f.display(),
            Value::String(s) => s.clone(),
            Value::Number(n) => {
                if n.fract() == 0.0 && n.is_finite() {
                    format!("{}", *n as i64)
                } else {
                    n.to_string()
                }
            }
            _ => self.to_display_string(),
        }
    }

    /// Возвращает строковое представление значения (для консоли).
    pub fn to_display_string(&self) -> String {
        match self {
            Value::Undefined => "undefined".to_string(),
            Value::Null => "null".to_string(),
            Value::Boolean(b) => b.to_string(),
            Value::Number(n) => {
                if n.fract() == 0.0 && n.is_finite() {
                    format!("{}", *n as i64)
                } else {
                    n.to_string()
                }
            }
            Value::String(s) => s.clone(),
            Value::Object(_) => "[object Object]".to_string(),
            Value::Array(items) => {
                let inner: Vec<String> = items.iter().map(|v| v.to_display_string()).collect();
                format!("[{}]", inner.join(", "))
            }
            Value::Function(f) => {
                if f.is_arrow {
                    format!("[Arrow Function: {}]", f.name)
                } else {
                    format!("[Function: {}]", f.name)
                }
            }
            Value::Promise(p) => {
                let p = p.borrow();
                match p.state {
                    PromiseState::Pending => "Promise { <pending> }".to_string(),
                    PromiseState::Fulfilled => {
                        let v = p.value.as_ref().map(|v| v.to_display_string()).unwrap_or_default();
                        format!("Promise {{ <fulfilled>: {} }}", v)
                    }
                    PromiseState::Rejected => {
                        let v = p.value.as_ref().map(|v| v.to_display_string()).unwrap_or_default();
                        format!("Promise {{ <rejected>: {} }}", v)
                    }
                }
            }
        }
    }

    /// Строгое сравнение (`===`) по спецификации ECMAScript.
    /// Сравнивает значения без приведения типов.
    pub fn strict_equals(&self, other: &Value) -> bool {
        match (self, other) {
            (Value::Number(a), Value::Number(b)) => a == b,
            (Value::String(a), Value::String(b)) => a == b,
            (Value::Boolean(a), Value::Boolean(b)) => a == b,
            (Value::Null, Value::Null) => true,
            (Value::Undefined, Value::Undefined) => true,
            (Value::Object(a), Value::Object(b)) => std::ptr::eq(a, b),
            (Value::Array(a), Value::Array(b)) => std::ptr::eq(a, b),
            (Value::Function(a), Value::Function(b)) => std::ptr::eq(a, b),
            (Value::Promise(a), Value::Promise(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }

    /// Проверяет, является ли значение "истинным" (truthy) в JS.
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Undefined | Value::Null => false,
            Value::Boolean(b) => *b,
            Value::Number(n) => *n != 0.0 && !n.is_nan(),
            Value::String(s) => !s.is_empty(),
            Value::Object(_) | Value::Array(_) | Value::Function(_) | Value::Promise(_) => true,
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Value::Undefined
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_number() {
        assert_eq!(Value::Number(42.0).to_display_string(), "42");
        assert_eq!(Value::Number(3.14).to_display_string(), "3.14");
    }

    #[test]
    fn test_truthy() {
        assert!(Value::Number(1.0).is_truthy());
        assert!(!Value::Number(0.0).is_truthy());
        assert!(!Value::Undefined.is_truthy());
        assert!(Value::String("hello".to_string()).is_truthy());
        assert!(!Value::String(String::new()).is_truthy());
    }
}