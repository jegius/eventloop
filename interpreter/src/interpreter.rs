//! Tree-walking интерпретатор.
//!
//! Исполняет AST. На данном этапе реализована базовая заготовка;
//! полная реализация (значения, окружения, функции, промисы) будет
//! выполнена на этапе 1 и 2 дорожной карты.

use crate::ast::{Expr, Program, Stmt};
use crate::console::Console;
use crate::environment::Environment;
use crate::event::EventLoopEvent;
use crate::queues::{
    AnimationFrameQueue, IdleCallbackQueue, MacrotaskQueue, MicrotaskQueue, RenderQueue,
};
use crate::value::Value;
use std::cell::RefCell;
use std::rc::Rc;

/// Ошибка выполнения.
#[derive(Debug, Clone, PartialEq)]
pub struct RuntimeError {
    pub message: String,
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// Интерпретатор.
pub struct Interpreter {
    /// Глобальное окружение.
    pub globals: Rc<RefCell<Environment>>,
    /// Консоль.
    pub console: Console,
    /// События Event Loop, генерируемые интерпретатором.
    pub events: Vec<EventLoopEvent>,
    /// Очередь микрозадач.
    pub microtask_queue: Rc<RefCell<MicrotaskQueue>>,
    /// Очередь макрозадач.
    pub macrotask_queue: Rc<RefCell<MacrotaskQueue>>,
    /// Очередь rAF.
    pub raf_queue: Rc<RefCell<AnimationFrameQueue>>,
    /// Очередь rIC.
    pub ric_queue: Rc<RefCell<IdleCallbackQueue>>,
    /// Очередь рендера.
    pub render_queue: Rc<RefCell<RenderQueue>>,
    /// Обработчики браузерных событий: тип события → список колбэков.
    pub event_listeners: std::collections::HashMap<String, Vec<Value>>,
    /// Счётчик ID таймеров (setTimeout/setInterval).
    pub timer_id: u64,
    /// Значение, переданное в resolve() внутри executor (для new Promise).
    pub promise_resolve_value: Option<Value>,
    /// Возвращаемое значение функции (для `return`).
    pub return_value: Option<Value>,
}

impl Interpreter {
    /// Создаёт новый интерпретатор.
    pub fn new() -> Self {
        let globals = Rc::new(RefCell::new(Environment::new()));
        let mut interpreter = Interpreter {
            globals: globals.clone(),
            console: Console::new(),
            events: Vec::new(),
            microtask_queue: Rc::new(RefCell::new(MicrotaskQueue::new())),
            macrotask_queue: Rc::new(RefCell::new(MacrotaskQueue::new())),
            raf_queue: Rc::new(RefCell::new(AnimationFrameQueue::new())),
            ric_queue: Rc::new(RefCell::new(IdleCallbackQueue::new())),
            render_queue: Rc::new(RefCell::new(RenderQueue::new())),
            event_listeners: std::collections::HashMap::new(),
            timer_id: 0,
            promise_resolve_value: None,
            return_value: None,
        };
        interpreter.install_builtins();
        interpreter
    }

    /// Устанавливает встроенные функции в глобальное окружение.
    fn install_builtins(&mut self) {
        let globals = self.globals.clone();

        // console объект
        let mut console_obj = crate::value::Object::default();
        console_obj.properties.insert(
            "log".to_string(),
            Value::Function(crate::value::Function {
                name: "console.log".to_string(),
                params: vec!["...args".to_string()],
                body: None,
                is_arrow: false,
            }),
        );
        globals.borrow_mut().define("console", Value::Object(console_obj));

        // setTimeout
        globals.borrow_mut().define(
            "setTimeout",
            Value::Function(crate::value::Function {
                name: "setTimeout".to_string(),
                params: vec!["callback".to_string(), "delay".to_string()],
                body: None,
                is_arrow: false,
            }),
        );

        // setInterval
        globals.borrow_mut().define(
            "setInterval",
            Value::Function(crate::value::Function {
                name: "setInterval".to_string(),
                params: vec!["callback".to_string(), "delay".to_string()],
                body: None,
                is_arrow: false,
            }),
        );

        // setImmediate — макрозадача, выполняется сразу после текущего синхронного кода.
        globals.borrow_mut().define(
            "setImmediate",
            Value::Function(crate::value::Function {
                name: "setImmediate".to_string(),
                params: vec!["callback".to_string()],
                body: None,
                is_arrow: false,
            }),
        );

        // Promise
        let mut promise_obj = crate::value::Object::default();
        promise_obj.properties.insert(
            "resolve".to_string(),
            Value::Function(crate::value::Function {
                name: "Promise.resolve".to_string(),
                params: vec!["value".to_string()],
                body: None,
                is_arrow: false,
            }),
        );
        globals.borrow_mut().define("Promise", Value::Object(promise_obj));

        // queueMicrotask
        globals.borrow_mut().define(
            "queueMicrotask",
            Value::Function(crate::value::Function {
                name: "queueMicrotask".to_string(),
                params: vec!["callback".to_string()],
                body: None,
                is_arrow: false,
            }),
        );

        // requestAnimationFrame
        globals.borrow_mut().define(
            "requestAnimationFrame",
            Value::Function(crate::value::Function {
                name: "requestAnimationFrame".to_string(),
                params: vec!["callback".to_string()],
                body: None,
                is_arrow: false,
            }),
        );

        // requestIdleCallback
        globals.borrow_mut().define(
            "requestIdleCallback",
            Value::Function(crate::value::Function {
                name: "requestIdleCallback".to_string(),
                params: vec!["callback".to_string()],
                body: None,
                is_arrow: false,
            }),
        );

        // document — объект с addEventListener / dispatchEvent
        let mut document_obj = crate::value::Object::default();
        document_obj.properties.insert(
            "addEventListener".to_string(),
            Value::Function(crate::value::Function {
                name: "document.addEventListener".to_string(),
                params: vec!["type".to_string(), "callback".to_string()],
                body: None,
                is_arrow: false,
            }),
        );
        document_obj.properties.insert(
            "dispatchEvent".to_string(),
            Value::Function(crate::value::Function {
                name: "document.dispatchEvent".to_string(),
                params: vec!["event".to_string()],
                body: None,
                is_arrow: false,
            }),
        );
        globals.borrow_mut().define("document", Value::Object(document_obj));

        // window — объект с addEventListener / dispatchEvent
        let mut window_obj = crate::value::Object::default();
        window_obj.properties.insert(
            "addEventListener".to_string(),
            Value::Function(crate::value::Function {
                name: "window.addEventListener".to_string(),
                params: vec!["type".to_string(), "callback".to_string()],
                body: None,
                is_arrow: false,
            }),
        );
        window_obj.properties.insert(
            "dispatchEvent".to_string(),
            Value::Function(crate::value::Function {
                name: "window.dispatchEvent".to_string(),
                params: vec!["event".to_string()],
                body: None,
                is_arrow: false,
            }),
        );
        globals.borrow_mut().define("window", Value::Object(window_obj));

        // fetch — асинхронная операция, возвращает Promise
        globals.borrow_mut().define(
            "fetch",
            Value::Function(crate::value::Function {
                name: "fetch".to_string(),
                params: vec!["url".to_string()],
                body: None,
                is_arrow: false,
            }),
        );
    }

    /// Выполняет программу.
    pub fn interpret(&mut self, program: &Program) -> Result<Value, RuntimeError> {
        let globals = self.globals.clone();
        for stmt in &program.statements {
            // Каждый оператор попадает в call stack отдельно.
            let label = stmt.display();
            let line = stmt.line();
            self.events.push(EventLoopEvent::CallStackPush(label.clone(), line, line));
            let result = self.execute_statement(stmt, &globals);
            self.events.push(EventLoopEvent::CallStackPop(label));
            result?;
        }
        Ok(Value::Undefined)
    }

    /// Выполняет оператор.
    fn execute_statement(
        &mut self,
        stmt: &Stmt,
        env: &Rc<RefCell<Environment>>,
    ) -> Result<Value, RuntimeError> {
        match stmt {
            Stmt::VarDecl { name, init, .. } => {
                let value = match init {
                    Some(expr) => self.evaluate(expr, env)?,
                    None => Value::Undefined,
                };
                env.borrow_mut().define(name, value);
                Ok(Value::Undefined)
            }
            Stmt::Expr { expr, .. } => {
                self.evaluate(expr, env)?;
                Ok(Value::Undefined)
            }
            Stmt::Block { statements, .. } => {
                let child_env = Rc::new(RefCell::new(Environment::with_parent(env.clone())));
                for s in statements {
                    self.execute_statement(s, &child_env)?;
                }
                Ok(Value::Undefined)
            }
            Stmt::If { cond, then_branch, else_branch, .. } => {
                let cond_value = self.evaluate(cond, env)?;
                if cond_value.is_truthy() {
                    self.execute_statement(then_branch, env)?;
                } else if let Some(else_branch) = else_branch {
                    self.execute_statement(else_branch, env)?;
                }
                Ok(Value::Undefined)
            }
            Stmt::While { cond, body, .. } => {
                while self.evaluate(cond, env)?.is_truthy() {
                    match self.execute_statement(body, env) {
                        Err(e) if e.message == "BREAK" => break,
                        Err(e) if e.message == "CONTINUE" => continue,
                        Err(e) => return Err(e),
                        Ok(_) => {}
                    }
                }
                Ok(Value::Undefined)
            }
            Stmt::For { init, cond, update, body, .. } => {
                if let Some(init) = init {
                    self.execute_statement(init, env)?;
                }
                while cond.as_ref().map(|c| self.evaluate(c, env).map(|v| v.is_truthy())).transpose()?.unwrap_or(true) {
                    match self.execute_statement(body, env) {
                        Err(e) if e.message == "BREAK" => break,
                        Err(e) if e.message == "CONTINUE" => {
                            if let Some(update) = update {
                                self.evaluate(update, env)?;
                            }
                            continue;
                        }
                        Err(e) => return Err(e),
                        Ok(_) => {}
                    }
                    if let Some(update) = update {
                        self.evaluate(update, env)?;
                    }
                }
                Ok(Value::Undefined)
            }
            Stmt::Return { expr, .. } => {
                let value = match expr {
                    Some(expr) => self.evaluate(expr, env)?,
                    None => Value::Undefined,
                };
                // Сохраняем возвращаемое значение, чтобы `execute_user_function`
                // могла его извлечь (для поддержки `return` в цепочках промисов).
                self.return_value = Some(value);
                Err(RuntimeError { message: "RETURN".to_string() })
            }
            Stmt::Break { .. } => Err(RuntimeError { message: "BREAK".to_string() }),
            Stmt::Continue { .. } => Err(RuntimeError { message: "CONTINUE".to_string() }),
            Stmt::FunctionDecl { name, params, body, is_async, .. } => {
                // Для async-функции имя в Value::Function помечается префиксом `async:`,
                // чтобы `call_function_with_this` распознала её как async.
                // В окружении функция доступна по исходному имени `name`.
                let func_name = if *is_async {
                    format!("async:{}", name)
                } else {
                    name.clone()
                };
                let func = Value::Function(crate::value::Function {
                    name: func_name,
                    params: params.clone(),
                    body: Some(body.clone()),
                    is_arrow: false,
                });
                env.borrow_mut().define(name, func);
                Ok(Value::Undefined)
            }
            Stmt::Throw { expr, .. } => {
                let value = self.evaluate(expr, env)?;
                Err(RuntimeError { message: format!("Uncaught: {}", value.to_display_string()) })
            }
            Stmt::Try { body, catch_param, catch_body, finally_body, .. } => {
                let result = self.execute_statement(body, env);
                if result.is_err() {
                    if let (Some(param), Some(catch_body)) = (catch_param, catch_body) {
                        let catch_env = Rc::new(RefCell::new(Environment::with_parent(env.clone())));
                        // Извлекаем значение ошибки из сообщения (после префикса "Uncaught: ").
                        let err_value = match &result {
                            Err(e) if e.message.starts_with("Uncaught: ") => {
                                Value::String(e.message.trim_start_matches("Uncaught: ").to_string())
                            }
                            Err(e) => Value::String(e.message.clone()),
                            _ => Value::Undefined,
                        };
                        catch_env.borrow_mut().define(param, err_value);
                        self.execute_statement(catch_body, &catch_env)?;
                    }
                }
                if let Some(finally_body) = finally_body {
                    self.execute_statement(finally_body, env)?;
                }
                Ok(Value::Undefined)
            }
            Stmt::Switch { expr, cases, default, .. } => {
                let value = self.evaluate(expr, env)?;
                let mut matched = false;
                let mut executed = false;
                for (case_value, body) in cases {
                    let case_val = self.evaluate(case_value, env)?;
                    // Строгое сравнение (===) по спецификации ECMAScript.
                    if !matched && case_val.strict_equals(&value) {
                        matched = true;
                    }
                    if matched {
                        for s in body {
                            self.execute_statement(s, env)?;
                        }
                        executed = true;
                    }
                }
                if !matched {
                    if let Some(default_body) = default {
                        for s in default_body {
                            self.execute_statement(s, env)?;
                        }
                        executed = true;
                    }
                }
                let _ = executed;
                Ok(Value::Undefined)
            }
            Stmt::ClassDecl { name, methods, .. } => {
                // Создаём объект-класс с методами. constructor — функция-конструктор.
                let mut class_obj = crate::value::Object::default();
                for (method_name, params, body) in methods {
                    let func = Value::Function(crate::value::Function {
                        name: format!("{}.{}", name, method_name),
                        params: params.clone(),
                        body: Some(body.clone()),
                        is_arrow: false,
                    });
                    class_obj.properties.insert(method_name.clone(), func);
                }
                env.borrow_mut().define(name, Value::Object(class_obj));
                Ok(Value::Undefined)
            }
        }
    }

    /// Вычисляет выражение.
    fn evaluate(&mut self, expr: &Expr, env: &Rc<RefCell<Environment>>) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::String(s) => Ok(Value::String(s.clone())),
            Expr::Boolean(b) => Ok(Value::Boolean(*b)),
            Expr::Null => Ok(Value::Null),
            Expr::Undefined => Ok(Value::Undefined),
            Expr::Identifier(name) => {
                env.borrow()
                    .get(name)
                    .ok_or_else(|| RuntimeError { message: format!("Переменная '{}' не определена", name) })
            }
            Expr::Assign { name, value } => {
                let value = self.evaluate(value, env)?;
                if !env.borrow_mut().assign(name, value.clone()) {
                    return Err(RuntimeError { message: format!("Переменная '{}' не определена", name) });
                }
                Ok(value)
            }
            Expr::SetMember { object, property, value } => {
                let obj = self.evaluate(object, env)?;
                let val = self.evaluate(value, env)?;
                let key = match property.as_ref() {
                    Expr::Identifier(name) => name.clone(),
                    Expr::String(s) => s.clone(),
                    _ => self.evaluate(property, env)?.to_display_string(),
                };
                match obj {
                    Value::Object(mut o) => {
                        o.properties.insert(key, val.clone());
                        Ok(val)
                    }
                    _ => Err(RuntimeError { message: "Присваивание свойству возможно только для объектов".to_string() }),
                }
            }
            Expr::Binary { left, op, right } => {
                let left = self.evaluate(left, env)?;
                let right = self.evaluate(right, env)?;
                self.apply_binary(&left, op, &right)
            }
            Expr::Unary { op, expr } => {
                let value = self.evaluate(expr, env)?;
                match op.as_str() {
                    "!" => Ok(Value::Boolean(!value.is_truthy())),
                    "-" => match value {
                        Value::Number(n) => Ok(Value::Number(-n)),
                        _ => Err(RuntimeError { message: "Унарный минус применим только к числам".to_string() }),
                    },
                    _ => Err(RuntimeError { message: format!("Неизвестный унарный оператор: {}", op) }),
                }
            }
            Expr::Logical { left, op, right } => {
                let left = self.evaluate(left, env)?;
                if op == "&&" {
                    if !left.is_truthy() {
                        return Ok(left);
                    }
                    self.evaluate(right, env)
                } else {
                    if left.is_truthy() {
                        return Ok(left);
                    }
                    self.evaluate(right, env)
                }
            }
            Expr::Ternary { cond, then_expr, else_expr } => {
                if self.evaluate(cond, env)?.is_truthy() {
                    self.evaluate(then_expr, env)
                } else {
                    self.evaluate(else_expr, env)
                }
            }
            Expr::Call { callee, args } => {
                // Определяем `this` для вызова метода (obj.method()).
                // Вычисляем объект один раз, чтобы не дублировать побочные эффекты.
                let mut this_value = None;
                let callee_value = if let Expr::Member { object, property } = callee.as_ref() {
                    let obj = self.evaluate(object, env)?;
                    this_value = Some(obj.clone());
                    let key = match property.as_ref() {
                        Expr::Identifier(name) => name.clone(),
                        Expr::String(s) => s.clone(),
                        _ => self.evaluate(property, env)?.to_display_string(),
                    };
                    match &obj {
                        Value::Object(o) => o
                            .properties
                            .get(&key)
                            .cloned()
                            .ok_or_else(|| RuntimeError { message: format!("Свойство '{}' не найдено", key) })?,
                        Value::Promise(_) if key == "then" => Value::Function(crate::value::Function {
                            name: "Promise.then".to_string(),
                            params: vec!["callback".to_string()],
                            body: None,
                            is_arrow: false,
                        }),
                        Value::Promise(_) if key == "catch" => Value::Function(crate::value::Function {
                            name: "Promise.catch".to_string(),
                            params: vec!["callback".to_string()],
                            body: None,
                            is_arrow: false,
                        }),
                        Value::Promise(_) if key == "finally" => Value::Function(crate::value::Function {
                            name: "Promise.finally".to_string(),
                            params: vec!["callback".to_string()],
                            body: None,
                            is_arrow: false,
                        }),
                        _ => return Err(RuntimeError { message: "Доступ к свойству возможен только для объектов".to_string() }),
                    }
                } else {
                    self.evaluate(callee, env)?
                };
                let mut arg_values = Vec::new();
                for arg in args {
                    arg_values.push(self.evaluate(arg, env)?);
                }
                self.call_function_with_this(&callee_value, &arg_values, env, this_value)
            }
            Expr::Member { object, property } => {
                let object = self.evaluate(object, env)?;
                // Для доступа к свойству используем имя идентификатора или строку как ключ
                let key = match property.as_ref() {
                    Expr::Identifier(name) => name.clone(),
                    Expr::String(s) => s.clone(),
                    _ => self.evaluate(property, env)?.to_display_string(),
                };
                match object {
                    Value::Object(obj) => obj
                        .properties
                        .get(&key)
                        .cloned()
                        .ok_or_else(|| RuntimeError { message: format!("Свойство '{}' не найдено", key) }),
                    Value::Array(items) => {
                        if let Ok(index) = key.parse::<usize>() {
                            items.get(index).cloned().ok_or_else(|| RuntimeError { message: "Индекс вне диапазона".to_string() })
                        } else {
                            Err(RuntimeError { message: "Некорректный индекс массива".to_string() })
                        }
                    }
                    Value::Promise(_) if key == "then" => {
                        // Promise.then — добавляет колбэк в microtask queue.
                        Ok(Value::Function(crate::value::Function {
                            name: "Promise.then".to_string(),
                            params: vec!["callback".to_string()],
                            body: None,
                            is_arrow: false,
                        }))
                    }
                    Value::Promise(_) if key == "catch" => {
                        // Promise.catch — добавляет колбэк обработки ошибки в microtask queue.
                        Ok(Value::Function(crate::value::Function {
                            name: "Promise.catch".to_string(),
                            params: vec!["callback".to_string()],
                            body: None,
                            is_arrow: false,
                        }))
                    }
                    Value::Promise(_) if key == "finally" => {
                        // Promise.finally — добавляет колбэк в microtask queue.
                        Ok(Value::Function(crate::value::Function {
                            name: "Promise.finally".to_string(),
                            params: vec!["callback".to_string()],
                            body: None,
                            is_arrow: false,
                        }))
                    }
                    _ => Err(RuntimeError { message: "Доступ к свойству возможен только для объектов".to_string() }),
                }
            }
            Expr::Object(properties) => {
                let mut obj = crate::value::Object::default();
                for (key, value_expr) in properties {
                    let value = self.evaluate(value_expr, env)?;
                    obj.properties.insert(key.clone(), value);
                }
                Ok(Value::Object(obj))
            }
            Expr::Array(items) => {
                let mut arr = Vec::new();
                for item in items {
                    arr.push(self.evaluate(item, env)?);
                }
                Ok(Value::Array(arr))
            }
            Expr::Function { name, params, body, is_arrow } => {
                Ok(Value::Function(crate::value::Function {
                    name: name.clone().unwrap_or_default(),
                    params: params.clone(),
                    body: Some(body.clone()),
                    is_arrow: *is_arrow,
                }))
            }
            Expr::This => {
                // Возвращаем `this` из окружения (для методов классов), иначе undefined.
                Ok(env.borrow().get("this").unwrap_or(Value::Undefined))
            }
            Expr::Typeof(expr) => {
                let value = self.evaluate(expr, env)?;
                let type_str = match value {
                    Value::Undefined => "undefined",
                    Value::Null => "object",
                    Value::Boolean(_) => "boolean",
                    Value::Number(_) => "number",
                    Value::String(_) => "string",
                    Value::Object(_) | Value::Array(_) => "object",
                    Value::Function(_) => "function",
                    Value::Promise(_) => "object",
                };
                Ok(Value::String(type_str.to_string()))
            }
            Expr::New { callee, args } => {
                let callee_value = self.evaluate(callee, env)?;
                let mut arg_values = Vec::new();
                for arg in args {
                    arg_values.push(self.evaluate(arg, env)?);
                }
                // Если callee — Promise (new Promise(executor)), создаём Promise и вызываем executor.
                if let Value::Object(promise_obj) = &callee_value {
                    if promise_obj.properties.contains_key("resolve") {
                        // Создаём Promise в состоянии Pending.
                        let promise = Rc::new(RefCell::new(crate::value::Promise {
                            state: crate::value::PromiseState::Pending,
                            value: None,
                            then_callbacks: Vec::new(),
                            catch_callbacks: Vec::new(),
                            is_fetch: false,
                        }));
                        // Функция resolve(value) — разрешает Promise переданным значением.
                        let resolve_fn = Value::Function(crate::value::Function {
                            name: "Promise.resolve".to_string(),
                            params: vec!["value".to_string()],
                            body: None,
                            is_arrow: false,
                        });
                        // Вызываем executor с функцией resolve.
                        if let Some(executor) = arg_values.first() {
                            let _ = self.call_function_with_this(
                                executor,
                                &[resolve_fn],
                                env,
                                None,
                            );
                        }
                        // Используем значение, переданное в resolve(value).
                        if let Some(v) = self.promise_resolve_value.take() {
                            promise.borrow_mut().value = Some(Box::new(v));
                        }
                        // Помечаем Promise как Fulfilled.
                        promise.borrow_mut().state = crate::value::PromiseState::Fulfilled;
                        if promise.borrow().value.is_none() {
                            promise.borrow_mut().value = Some(Box::new(Value::Undefined));
                        }
                        return Ok(Value::Promise(promise));
                    }
                }
                // Если callee — объект-класс, создаём экземпляр
                if let Value::Object(class_obj) = &callee_value {
                    let mut instance = crate::value::Object::default();
                    // Копируем методы класса в экземпляр
                    for (k, v) in &class_obj.properties {
                        instance.properties.insert(k.clone(), v.clone());
                    }
                    // Вызываем constructor с привязкой `this` к экземпляру
                    if let Some(ctor) = instance.properties.get("constructor").cloned() {
                        let this_val = Value::Object(instance.clone());
                        let _ = self.call_function_with_this(&ctor, &arg_values, env, Some(this_val));
                    }
                    return Ok(Value::Object(instance));
                }
                self.call_function(&callee_value, &arg_values, env)
            }
            Expr::Await(expr) => {
                let value = self.evaluate(expr, env)?;
                // await на Promise: возвращаем значение, если fulfilled
                let result = match value {
                    Value::Promise(p) => {
                        let p = p.borrow();
                        match p.state {
                            crate::value::PromiseState::Fulfilled => {
                                p.value.clone().map(|v| *v).unwrap_or(Value::Undefined)
                            }
                            crate::value::PromiseState::Rejected => {
                                p.value.clone().map(|v| *v).unwrap_or(Value::Undefined)
                            }
                            crate::value::PromiseState::Pending => Value::Undefined,
                        }
                    }
                    other => other,
                };
                Ok(result)
            }
            Expr::AsyncFunction { name, params, body } => {
                Ok(Value::Function(crate::value::Function {
                    name: name.clone().unwrap_or_default(),
                    params: params.clone(),
                    body: Some(body.clone()),
                    is_arrow: false,
                }))
            }
        }
    }

    /// Выполняет бинарную операцию.
    fn apply_binary(&self, left: &Value, op: &str, right: &Value) -> Result<Value, RuntimeError> {
        match op {
            "+" => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
                (Value::String(a), b) => Ok(Value::String(format!("{}{}", a, b.to_display_string()))),
                (a, Value::String(b)) => Ok(Value::String(format!("{}{}", a.to_display_string(), b))),
                _ => Err(RuntimeError { message: "Оператор '+' применим к числам и строкам".to_string() }),
            },
            "-" => self.number_op(left, right, |a, b| a - b),
            "*" => self.number_op(left, right, |a, b| a * b),
            "/" => self.number_op(left, right, |a, b| a / b),
            "%" => self.number_op(left, right, |a, b| a % b),
            "==" => Ok(Value::Boolean(self.loose_equals(left, right))),
            "!=" => Ok(Value::Boolean(!self.loose_equals(left, right))),
            "===" => Ok(Value::Boolean(left.strict_equals(right))),
            "!==" => Ok(Value::Boolean(!left.strict_equals(right))),
            "<" => self.number_op(left, right, |a, b| if a < b { 1.0 } else { 0.0 }).map(|v| Value::Boolean(v == Value::Number(1.0))),
            ">" => self.number_op(left, right, |a, b| if a > b { 1.0 } else { 0.0 }).map(|v| Value::Boolean(v == Value::Number(1.0))),
            "<=" => self.number_op(left, right, |a, b| if a <= b { 1.0 } else { 0.0 }).map(|v| Value::Boolean(v == Value::Number(1.0))),
            ">=" => self.number_op(left, right, |a, b| if a >= b { 1.0 } else { 0.0 }).map(|v| Value::Boolean(v == Value::Number(1.0))),
            _ => Err(RuntimeError { message: format!("Неизвестный бинарный оператор: {}", op) }),
        }
    }

    fn number_op<F>(&self, left: &Value, right: &Value, op: F) -> Result<Value, RuntimeError>
    where
        F: Fn(f64, f64) -> f64,
    {
        match (left, right) {
            (Value::Number(a), Value::Number(b)) => Ok(Value::Number(op(*a, *b))),
            _ => Err(RuntimeError { message: "Оператор применим только к числам".to_string() }),
        }
    }

    /// Нестрогое сравнение (`==`) по спецификации ECMAScript.
    ///
    /// Выполняет приведение типов перед сравнением. Реализует базовые правила:
    /// - если типы совпадают — строгое сравнение;
    /// - `null == undefined` → true;
    /// - число и строка — строка приводится к числу;
    /// - boolean приводится к числу (true → 1, false → 0);
    /// - объект и примитив — объект приводится к примитиву (для простоты — к строке).
    fn loose_equals(&self, left: &Value, right: &Value) -> bool {
        // Если типы совпадают — используем строгое сравнение.
        if std::mem::discriminant(left) == std::mem::discriminant(right) {
            return left.strict_equals(right);
        }

        // null == undefined → true (и наоборот).
        match (left, right) {
            (Value::Null, Value::Undefined) | (Value::Undefined, Value::Null) => return true,
            _ => {}
        }

        // Приводим boolean к числу.
        let to_number = |v: &Value| -> Option<f64> {
            match v {
                Value::Number(n) => Some(*n),
                Value::Boolean(b) => Some(if *b { 1.0 } else { 0.0 }),
                Value::String(s) => s.trim().parse::<f64>().ok(),
                _ => None,
            }
        };

        // Число и строка / boolean — приводим к числу.
        if let (Some(a), Some(b)) = (to_number(left), to_number(right)) {
            return a == b;
        }

        // Объект и примитив — для простоты сравниваем строковые представления.
        match (left, right) {
            (Value::Object(_), _) | (_, Value::Object(_)) => {
                left.to_display_string() == right.to_display_string()
            }
            (Value::Array(_), _) | (_, Value::Array(_)) => {
                left.to_display_string() == right.to_display_string()
            }
            _ => false,
        }
    }

    /// Вызывает функцию.
    pub fn call_function(
        &mut self,
        callee: &Value,
        args: &[Value],
        env: &Rc<RefCell<Environment>>,
    ) -> Result<Value, RuntimeError> {
        self.call_function_with_this(callee, args, env, None)
    }

    /// Вызывает функцию с привязкой `this` (для методов классов).
    pub fn call_function_with_this(
        &mut self,
        callee: &Value,
        args: &[Value],
        env: &Rc<RefCell<Environment>>,
        this_value: Option<Value>,
    ) -> Result<Value, RuntimeError> {
        match callee {
            Value::Function(func) => {
                // Встроенные функции
                match func.name.as_str() {
                    "console.log" => {
                        let msg: Vec<String> = args.iter().map(|a| a.to_display_string()).collect();
                        let line = msg.join(" ");
                        self.console.log(&line);
                        self.events.push(EventLoopEvent::ConsoleLog(line));
                        Ok(Value::Undefined)
                    }
                    "setTimeout" => {
                        if let Some(callback) = args.first() {
                            self.timer_id += 1;
                            let id = self.timer_id;
                            let label = callback.display();
                            self.macrotask_queue
                                .borrow_mut()
                                .enqueue(&label, callback.clone(), Vec::new());
                            self.events.push(EventLoopEvent::MacrotaskEnqueue(label));
                            return Ok(Value::Number(id as f64));
                        }
                        Ok(Value::Undefined)
                    }
                    "setInterval" => {
                        if let Some(callback) = args.first() {
                            self.timer_id += 1;
                            let id = self.timer_id;
                            let label = callback.display();
                            self.macrotask_queue
                                .borrow_mut()
                                .enqueue(&label, callback.clone(), Vec::new());
                            self.events.push(EventLoopEvent::MacrotaskEnqueue(label));
                            return Ok(Value::Number(id as f64));
                        }
                        Ok(Value::Undefined)
                    }
                    "setImmediate" => {
                        // setImmediate — макрозадача, выполняется сразу после текущего синхронного кода.
                        if let Some(callback) = args.first() {
                            self.timer_id += 1;
                            let id = self.timer_id;
                            let label = callback.display();
                            self.macrotask_queue
                                .borrow_mut()
                                .enqueue(&label, callback.clone(), Vec::new());
                            self.events.push(EventLoopEvent::MacrotaskEnqueue(label));
                            return Ok(Value::Number(id as f64));
                        }
                        Ok(Value::Undefined)
                    }
                    "queueMicrotask" => {
                        if let Some(callback) = args.first() {
                            self.microtask_queue
                                .borrow_mut()
                                .enqueue("queueMicrotask", callback.clone(), Vec::new());
                            self.events
                                .push(EventLoopEvent::MicrotaskEnqueue("queueMicrotask".to_string()));
                        }
                        Ok(Value::Undefined)
                    }
                    "requestAnimationFrame" => {
                        if let Some(callback) = args.first() {
                            self.raf_queue
                                .borrow_mut()
                                .enqueue("requestAnimationFrame", callback.clone());
                            self.events
                                .push(EventLoopEvent::RafEnqueue("requestAnimationFrame".to_string()));
                        }
                        Ok(Value::Undefined)
                    }
                    "requestIdleCallback" => {
                        if let Some(callback) = args.first() {
                            self.ric_queue
                                .borrow_mut()
                                .enqueue("requestIdleCallback", callback.clone());
                            self.events
                                .push(EventLoopEvent::RicEnqueue("requestIdleCallback".to_string()));
                        }
                        Ok(Value::Undefined)
                    }
                    "Promise.resolve" => {
                        // Возвращаем значение в состоянии fulfilled Fulfilled с переданным значением.
                        let value = args.first().cloned().unwrap_or(Value::Undefined);
                        // Сохраняем значение для new Promise (executor вызывает resolve(value)).
                        self.promise_resolve_value = Some(value.clone());
                        Ok(Value::Promise(Rc::new(RefCell::new(crate::value::Promise {
                            state: crate::value::PromiseState::Fulfilled,
                            value: Some(Box::new(value)),
                            then_callbacks: Vec::new(),
                            catch_callbacks: Vec::new(),
                            is_fetch: false,
                        }))))
                    }
                    "Promise.then" => {
                        // .then(callback) — добавляет колбэк в microtask queue.
                        // Возвращает новый Promise для поддержки цепочек `.then().then()`.
                        // Значение Promise передаётся колбэку при выполнении.
                        if let Some(callback) = args.first() {
                            // Создаём новый Promise для цепочки.
                            let new_promise = Rc::new(RefCell::new(crate::value::Promise {
                                state: crate::value::PromiseState::Pending,
                                value: None,
                                then_callbacks: Vec::new(),
                                catch_callbacks: Vec::new(),
                                is_fetch: false,
                            }));

                            match this_value {
                                Some(Value::Promise(p)) => {
                                    if p.borrow().state == crate::value::PromiseState::Fulfilled {
                                        // Promise уже fulfilled — добавляем колбэк в microtask queue.
                                        // Значение Promise передаётся колбэку как единственный аргумент,
                                        // а `new_promise` привязывается через `next_promise` (для batching цепочки).
                                        let promise_value = p.borrow().value.clone().map(|v| *v).unwrap_or(Value::Undefined);
                                        let label = callback.display();
                                        self.microtask_queue
                                            .borrow_mut()
                                            .enqueue_with_next(&label, callback.clone(), vec![promise_value], Value::Promise(new_promise.clone()));
                                        self.events
                                            .push(EventLoopEvent::MicrotaskEnqueue(label));
                                    } else {
                                        // Promise Pending — добавляем колбэк в then_callbacks.
                                        p.borrow_mut().then_callbacks.push(callback.clone());
                                    }
                                }
                                _ => {
                                    // this не Promise — добавляем колбэк в microtask queue.
                                    let label = callback.display();
                                    self.microtask_queue
                                        .borrow_mut()
                                        .enqueue_with_next(&label, callback.clone(), vec![Value::Undefined], Value::Promise(new_promise.clone()));
                                    self.events
                                        .push(EventLoopEvent::MicrotaskEnqueue(label));
                                }
                            }
                            return Ok(Value::Promise(new_promise));
                        }
                        Ok(Value::Undefined)
                    }
                    "Promise.catch" => {
                        // .catch(callback) — добавляет колбэк обработки ошибки в microtask queue.
                        if let Some(callback) = args.first() {
                            // Извлекаем значение из `this` (Promise).
                            let promise_value = match this_value {
                                Some(Value::Promise(p)) => {
                                    p.borrow().value.clone().map(|v| *v).unwrap_or(Value::Undefined)
                                }
                                _ => Value::Undefined,
                            };
                            let label = callback.display();
                            self.microtask_queue
                                .borrow_mut()
                                .enqueue(&label, callback.clone(), vec![promise_value]);
                            self.events
                                .push(EventLoopEvent::MicrotaskEnqueue(label));
                        }
                        Ok(Value::Undefined)
                    }
                    "Promise.finally" => {
                        // .finally(callback) — добавляет колбэк в microtask queue.
                        // Колбэк выполняется независимо от состояния промиса.
                        if let Some(callback) = args.first() {
                            let promise_value = match this_value {
                                Some(Value::Promise(p)) => {
                                    p.borrow().value.clone().map(|v| *v).unwrap_or(Value::Undefined)
                                }
                                _ => Value::Undefined,
                            };
                            let label = callback.display();
                            self.microtask_queue
                                .borrow_mut()
                                .enqueue(&label, callback.clone(), vec![promise_value]);
                            self.events
                                .push(EventLoopEvent::MicrotaskEnqueue(label));
                        }
                        Ok(Value::Undefined)
                    }
                    "document.addEventListener" | "window.addEventListener" => {
                        // Регистрируем обработчик события: (type, callback)
                        if let (Some(Value::String(evt_type)), Some(callback)) =
                            (args.first(), args.get(1))
                        {
                            self.event_listeners
                                .entry(evt_type.clone())
                                .or_default()
                                .push(callback.clone());
                        }
                        Ok(Value::Undefined)
                    }
                    "document.dispatchEvent" | "window.dispatchEvent" => {
                        // Диспатчим событие: (event) — event.type определяет обработчики
                        if let Some(Value::Object(evt)) = args.first() {
                            let evt_type = evt
                                .properties
                                .get("type")
                                .map(|v| v.to_display_string())
                                .unwrap_or_default();
                            // Добавляем обработчики события в macrotask queue
                            if let Some(callbacks) = self.event_listeners.get(&evt_type).cloned() {
                                for cb in callbacks {
                                    self.macrotask_queue
                                        .borrow_mut()
                                        .enqueue(&format!("event:{}", evt_type), cb.clone(), Vec::new());
                                    self.events.push(EventLoopEvent::MacrotaskEnqueue(
                                        format!("event:{}", evt_type),
                                    ));
                                }
                            }
                        }
                        Ok(Value::Undefined)
                    }
                    "fetch" => {
                        // fetch(url) — возвращает Promise, разрешающийся как обычный промис.
                        // Создаём Promise в состоянии Fulfilled с ответом.
                        let url = args
                            .first()
                            .map(|v| v.to_display_string())
                            .unwrap_or_default();
                        let mut response = crate::value::Object::default();
                        response.properties.insert(
                            "url".to_string(),
                            Value::String(url.clone()),
                        );
                        response.properties.insert(
                            "ok".to_string(),
                            Value::Boolean(true),
                        );
                        response.properties.insert(
                            "status".to_string(),
                            Value::Number(200.0),
                        );
                        let promise = Value::Promise(Rc::new(RefCell::new(crate::value::Promise {
                            state: crate::value::PromiseState::Fulfilled,
                            value: Some(Box::new(Value::Object(response))),
                            then_callbacks: Vec::new(),
                            catch_callbacks: Vec::new(),
                            is_fetch: true,
                        })));
                        Ok(promise)
                    }
                    _ => {
                        // Пользовательская функция
                        let is_async = func.name.starts_with("async:");
                        let result = self.execute_user_function(func, args, env, this_value);
                        if is_async {
                            // async-функция всегда возвращает Promise.
                            let value = match result {
                                Ok(v) => v,
                                Err(e) => {
                                    return Err(e);
                                }
                            };
                            return Ok(Value::Promise(Rc::new(RefCell::new(crate::value::Promise {
                                state: crate::value::PromiseState::Fulfilled,
                                value: Some(Box::new(value)),
                                then_callbacks: Vec::new(),
                                catch_callbacks: Vec::new(),
                                is_fetch: false,
                            }))));
                        }
                        result
                    }
                }
            }
            _ => Err(RuntimeError { message: "Вызов возможен только для функций".to_string() }),
        }
    }

    /// Выполняет пользовательскую функцию.
    fn execute_user_function(
        &mut self,
        func: &crate::value::Function,
        args: &[Value],
        env: &Rc<RefCell<Environment>>,
        this_value: Option<Value>,
    ) -> Result<Value, RuntimeError> {
        // Создаём новое окружение с родителем
        let func_env = Rc::new(RefCell::new(Environment::with_parent(env.clone())));

        // Привязываем `this` (для методов классов)
        if let Some(this) = this_value {
            func_env.borrow_mut().define("this", this);
        }

        // Связываем параметры с аргументами
        for (i, param) in func.params.iter().enumerate() {
            let value = args.get(i).cloned().unwrap_or(Value::Undefined);
            func_env.borrow_mut().define(param, value);
        }

        // Сбрасываем возвращаемое значение перед выполнением.
        self.return_value = None;

        // Выполняем тело функции (если оно есть).
        // Для async-функций используем специальную обработку `await`.
        let is_async = func.name.starts_with("async:");
        if let Some(body) = &func.body {
            if is_async {
                self.execute_async_body(body, &func_env)?;
            } else {
                match self.execute_statement(body, &func_env) {
                    // `return` — нормальное завершение функции.
                    Err(e) if e.message == "RETURN" => {}
                    Err(e) => return Err(e),
                    Ok(_) => {}
                }
            }
        }

        // Возвращаем значение из `return`, если оно было.
        Ok(self.return_value.take().unwrap_or(Value::Undefined))
    }

    /// Выполняет тело async-функции с поддержкой `await`.
    ///
    /// При встрече оператора `await` оставшаяся часть тела ставится в микротаску,
    /// что соответствует поведению браузера (продолжение после `await` — микротаска).
    fn execute_async_body(
        &mut self,
        body: &Stmt,
        env: &Rc<RefCell<Environment>>,
    ) -> Result<Value, RuntimeError> {
        // Извлекаем список операторов из блока.
        let statements: Vec<Stmt> = match body {
            Stmt::Block { statements, .. } => statements.clone(),
            other => vec![other.clone()],
        };

        for (i, stmt) in statements.iter().enumerate() {
            // Обрабатываем `await` как выражение-оператор.
            if let Stmt::Expr { expr: Expr::Await(inner), .. } = stmt {
                // Вычисляем значение, переданное в `await` (Promise).
                let value = self.evaluate(inner, env)?;
                // Оставшиеся операторы ставим в микротаску.
                let remaining = statements[i + 1..].to_vec();
                if !remaining.is_empty() {
                    let continuation = Value::Function(crate::value::Function {
                        name: "async:continuation".to_string(),
                        params: vec![],
                        body: Some(Box::new(Stmt::Block { statements: remaining, line: 0 })),
                        is_arrow: false,
                    });
                    self.microtask_queue
                        .borrow_mut()
                        .enqueue("async:await", continuation, vec![value]);
                    self.events
                        .push(EventLoopEvent::MicrotaskEnqueue("async:await".to_string()));
                }
                return Ok(Value::Undefined);
            }
            self.execute_statement(stmt, env)?;
        }
        Ok(Value::Undefined)
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_interpret_basic() {
        let mut parser = crate::parser::Parser::new("let x = 1 + 2;").unwrap();
        let program = parser.parse_program().unwrap();
        let mut interpreter = Interpreter::new();
        let result = interpreter.interpret(&program).unwrap();
        assert_eq!(result, Value::Undefined);
    }

    #[test]
    fn test_console_log() {
        let mut parser = crate::parser::Parser::new("console.log('hello');").unwrap();
        let program = parser.parse_program().unwrap();
        let mut interpreter = Interpreter::new();
        let _ = interpreter.interpret(&program);
        assert_eq!(interpreter.console.get_output(), vec!["hello"]);
    }

    #[test]
    fn test_set_timeout() {
        let mut parser = crate::parser::Parser::new("setTimeout(() => {}, 0);").unwrap();
        let program = parser.parse_program().unwrap();
        let mut interpreter = Interpreter::new();
        let _ = interpreter.interpret(&program);
        assert_eq!(interpreter.macrotask_queue.borrow().len(), 1);
    }

    #[test]
    fn test_strict_equals() {
        // 1 === '1' → false (строгое сравнение без приведения типов)
        let mut parser = crate::parser::Parser::new("console.log(1 === '1');").unwrap();
        let program = parser.parse_program().unwrap();
        let mut interpreter = Interpreter::new();
        let _ = interpreter.interpret(&program);
        assert_eq!(interpreter.console.get_output(), vec!["false"]);

        // 1 == '1' → true (нестрогое сравнение с приведением типов)
        let mut parser = crate::parser::Parser::new("console.log(1 == '1');").unwrap();
        let program = parser.parse_program().unwrap();
        let mut interpreter = Interpreter::new();
        let _ = interpreter.interpret(&program);
        assert_eq!(interpreter.console.get_output(), vec!["true"]);
    }

    #[test]
    fn test_break_continue() {
        // break в цикле
        let mut parser = crate::parser::Parser::new(
            "let sum = 0; for (let i = 0; i < 10; i++) { if (i === 3) break; sum = sum + i; } console.log(sum);",
        )
        .unwrap();
        let program = parser.parse_program().unwrap();
        let mut interpreter = Interpreter::new();
        let _ = interpreter.interpret(&program);
        assert_eq!(interpreter.console.get_output(), vec!["3"]);

        // continue в цикле (пропускаем чётные)
        let mut parser = crate::parser::Parser::new(
            "let sum = 0; for (let i = 0; i < 5; i++) { if (i % 2 === 0) continue; sum = sum + i; } console.log(sum);",
        )
        .unwrap();
        let program = parser.parse_program().unwrap();
        let mut interpreter = Interpreter::new();
        let _ = interpreter.interpret(&program);
        assert_eq!(interpreter.console.get_output(), vec!["4"]);
    }

    #[test]
    fn test_promise_catch() {
        // .catch() добавляет колбэк в microtask queue.
        let mut parser = crate::parser::Parser::new(
            "let p = new Promise((resolve) => { resolve(42); }); p.catch(() => {});",
        )
        .unwrap();
        let program = parser.parse_program().unwrap();
        let mut interpreter = Interpreter::new();
        let _ = interpreter.interpret(&program);
        assert_eq!(interpreter.microtask_queue.borrow().len(), 1);
    }

    #[test]
    fn test_promise_finally() {
        // .finally() добавляет колбэк в microtask queue.
        let mut parser = crate::parser::Parser::new(
            "let p = new Promise((resolve) => { resolve(42); }); p.finally(() => {});",
        )
        .unwrap();
        let program = parser.parse_program().unwrap();
        let mut interpreter = Interpreter::new();
        let _ = interpreter.interpret(&program);
        assert_eq!(interpreter.microtask_queue.borrow().len(), 1);
    }

    #[test]
    fn test_set_immediate() {
        // setImmediate добавляет макрозадачу в macrotask queue.
        let mut parser = crate::parser::Parser::new("setImmediate(() => {});").unwrap();
        let program = parser.parse_program().unwrap();
        let mut interpreter = Interpreter::new();
        let _ = interpreter.interpret(&program);
        assert_eq!(interpreter.macrotask_queue.borrow().len(), 1);
    }

    #[test]
    fn test_try_catch_value() {
        // try/catch передаёт реальное значение ошибки в catch-параметр.
        let mut parser = crate::parser::Parser::new(
            "try { throw 'boom'; } catch (e) { console.log(e); }",
        )
        .unwrap();
        let program = parser.parse_program().unwrap();
        let mut interpreter = Interpreter::new();
        let _ = interpreter.interpret(&program);
        assert_eq!(interpreter.console.get_output(), vec!["boom"]);
    }
}