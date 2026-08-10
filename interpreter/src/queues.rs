//! Очереди Event Loop.
//!
//! Реализует очереди, используемые механизмом Event Loop:
//! - Microtask Queue (Promise, queueMicrotask).
//! - Macrotask Queue (setTimeout, setInterval, I/O).
//! - Render Queue (задачи рендера).
//! - requestAnimationFrame (rAF).
//! - requestIdleCallback (rIC).

use crate::value::Value;
use std::collections::VecDeque;

/// Задача в очереди.
#[derive(Debug, Clone, PartialEq)]
pub struct Task {
    /// Идентификатор задачи (для визуализации).
    pub id: u64,
    /// Описание задачи (для визуализации).
    pub label: String,
    /// Колбэк (функция) для выполнения.
    pub callback: Value,
    /// Аргументы для колбэка.
    pub args: Vec<Value>,
    /// Следующий Promise в цепочке `.then().then()` (для batching микрозадач).
    pub next_promise: Option<Value>,
}

impl Task {
    /// Возвращает диапазон строк `(start, end)` колбэка в исходном коде
    /// (1-based, 0 — неизвестно).
    ///
    /// Диапазон извлекается из тела функции-колбэка (AST) и покрывает все
    /// строки тела колбэка целиком. Для встроенных функций (например,
    /// `console.log`) тело отсутствует, поэтому возвращается (0, 0).
    pub fn callback_line_range(&self) -> (usize, usize) {
        match &self.callback {
            Value::Function(f) => f.body.as_ref().map(|b| b.line_range()).unwrap_or((0, 0)),
            _ => (0, 0),
        }
    }
}

/// Очередь микрозадач (microtask queue).
#[derive(Debug, Clone, Default)]
pub struct MicrotaskQueue {
    tasks: VecDeque<Task>,
    next_id: u64,
}

impl MicrotaskQueue {
    /// Создаёт новую очередь микрозадач.
    pub fn new() -> Self {
        MicrotaskQueue { tasks: VecDeque::new(), next_id: 0 }
    }

    /// Добавляет микрозадачу в конец очереди.
    pub fn enqueue(&mut self, label: &str, callback: Value, args: Vec<Value>) -> u64 {
        self.next_id += 1;
        let task = Task {
            id: self.next_id,
            label: label.to_string(),
            callback,
            args,
            next_promise: None,
        };
        self.tasks.push_back(task);
        self.next_id
    }

    /// Добавляет микрозадачу с привязкой к следующему Promise в цепочке `.then().then()`.
    ///
    /// Поле `next_promise` используется для batching микрозадач: результат колбэка
    /// разрешает `next_promise`, а его `then_callbacks` добавляются в очередь.
    /// Это позволяет не передавать внутренний Promise как аргумент пользовательского
    /// колбэка (например, `console.log`).
    pub fn enqueue_with_next(
        &mut self,
        label: &str,
        callback: Value,
        args: Vec<Value>,
        next_promise: Value,
    ) -> u64 {
        self.next_id += 1;
        let task = Task {
            id: self.next_id,
            label: label.to_string(),
            callback,
            args,
            next_promise: Some(next_promise),
        };
        self.tasks.push_back(task);
        self.next_id
    }

    /// Извлекает микрозадачу из начала очереди.
    pub fn dequeue(&mut self) -> Option<Task> {
        self.tasks.pop_front()
    }

    /// Проверяет, пуста ли очередь.
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    /// Возвращает количество задач.
    pub fn len(&self) -> usize {
        self.tasks.len()
    }
}

/// Очередь макрозадач (macrotask queue).
#[derive(Debug, Clone, Default)]
pub struct MacrotaskQueue {
    tasks: VecDeque<Task>,
    next_id: u64,
}

impl MacrotaskQueue {
    /// Создаёт новую очередь макрозадач.
    pub fn new() -> Self {
        MacrotaskQueue { tasks: VecDeque::new(), next_id: 0 }
    }

    /// Добавляет макрозадачу в конец очереди.
    pub fn enqueue(&mut self, label: &str, callback: Value, args: Vec<Value>) -> u64 {
        self.next_id += 1;
        let task = Task {
            id: self.next_id,
            label: label.to_string(),
            callback,
            args,
            next_promise: None,
        };
        self.tasks.push_back(task);
        self.next_id
    }

    /// Извлекает макрозадачу из начала очереди.
    pub fn dequeue(&mut self) -> Option<Task> {
        self.tasks.pop_front()
    }

    /// Проверяет, пуста ли очередь.
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    /// Возвращает количество задач.
    pub fn len(&self) -> usize {
        self.tasks.len()
    }
}

/// Очередь requestAnimationFrame (rAF).
#[derive(Debug, Clone, Default)]
pub struct AnimationFrameQueue {
    tasks: VecDeque<Task>,
    next_id: u64,
}

impl AnimationFrameQueue {
    /// Создаёт новую очередь rAF.
    pub fn new() -> Self {
        AnimationFrameQueue { tasks: VecDeque::new(), next_id: 0 }
    }

    /// Добавляет колбэк rAF.
    pub fn enqueue(&mut self, label: &str, callback: Value) -> u64 {
        self.next_id += 1;
        let task = Task {
            id: self.next_id,
            label: label.to_string(),
            callback,
            args: Vec::new(),
            next_promise: None,
        };
        self.tasks.push_back(task);
        self.next_id
    }

    /// Извлекает колбэк rAF.
    pub fn dequeue(&mut self) -> Option<Task> {
        self.tasks.pop_front()
    }

    /// Проверяет, пуста ли очередь.
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
}

/// Очередь requestIdleCallback (rIC).
#[derive(Debug, Clone, Default)]
pub struct IdleCallbackQueue {
    tasks: VecDeque<Task>,
    next_id: u64,
}

impl IdleCallbackQueue {
    /// Создаёт новую очередь rIC.
    pub fn new() -> Self {
        IdleCallbackQueue { tasks: VecDeque::new(), next_id: 0 }
    }

    /// Добавляет колбэк rIC.
    pub fn enqueue(&mut self, label: &str, callback: Value) -> u64 {
        self.next_id += 1;
        let task = Task {
            id: self.next_id,
            label: label.to_string(),
            callback,
            args: Vec::new(),
            next_promise: None,
        };
        self.tasks.push_back(task);
        self.next_id
    }

    /// Извлекает колбэк rIC.
    pub fn dequeue(&mut self) -> Option<Task> {
        self.tasks.pop_front()
    }

    /// Проверяет, пуста ли очередь.
    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }
}

/// Очередь рендера (render queue).
#[derive(Debug, Clone, Default)]
pub struct RenderQueue {
    /// Флаги необходимости рендера.
    needs_render: bool,
}

impl RenderQueue {
    /// Создаёт новую очередь рендера.
    pub fn new() -> Self {
        RenderQueue { needs_render: false }
    }

    /// Помечает, что требуется рендер.
    pub fn request_render(&mut self) {
        self.needs_render = true;
    }

    /// Проверяет, требуется ли рендер.
    pub fn needs_render(&self) -> bool {
        self.needs_render
    }

    /// Сбрасывает флаг рендера.
    pub fn reset(&mut self) {
        self.needs_render = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_callback() -> Value {
        Value::Function(crate::value::Function {
            name: "cb".to_string(),
            params: Vec::new(),
            body: None,
            is_arrow: false,
        })
    }

    #[test]
    fn test_microtask_queue() {
        let mut q = MicrotaskQueue::new();
        q.enqueue("task1", dummy_callback(), Vec::new());
        q.enqueue("task2", dummy_callback(), Vec::new());
        assert_eq!(q.len(), 2);
        let t = q.dequeue().unwrap();
        assert_eq!(t.label, "task1");
        assert_eq!(q.len(), 1);
    }

    #[test]
    fn test_render_queue() {
        let mut q = RenderQueue::new();
        assert!(!q.needs_render());
        q.request_render();
        assert!(q.needs_render());
        q.reset();
        assert!(!q.needs_render());
    }
}