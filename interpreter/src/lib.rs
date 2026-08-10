//! Интерпретатор JavaScript с реализацией Event Loop по стандарту ECMAScript 2026.
//!
//! Этот крейт компилируется в WebAssembly и предоставляет API для фронтенда:
//! - `run` — выполнить JS-код и вернуть трассировку событий Event Loop.
//! - `step` — выполнить один шаг Event Loop (для пошагового режима).
//!
//! Модули:
//! - [`lexer`] — лексический анализатор.
//! - [`parser`] — синтаксический анализатор (AST).
//! - [`ast`] — определения узлов AST.
//! - [`interpreter`] — tree-walking интерпретатор.
//! - [`value`] — представление значений JS.
//! - [`environment`] — лексические окружения.
//! - [`event_loop`] — реализация Event Loop.
//! - [`queues`] — очереди (micro/macro/render/rAF/rIC).
//! - [`renderer`] — минимальный рендер-движок.
//! - [`console`] — сбор вывода.

pub mod ast;
pub mod console;
pub mod environment;
pub mod event;
pub mod event_loop;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod queues;
pub mod renderer;
pub mod value;

use wasm_bindgen::prelude::*;

/// Результат выполнения кода: трасса событий Event Loop и вывод консоли.
#[wasm_bindgen]
pub struct RunResult {
    events: Vec<String>,
    console_output: Vec<String>,
}

#[wasm_bindgen]
impl RunResult {
    /// События Event Loop в JSON-формате.
    #[wasm_bindgen(getter)]
    pub fn events(&self) -> Vec<String> {
        self.events.clone()
    }

    /// Вывод консоли.
    #[wasm_bindgen(getter)]
    pub fn console_output(&self) -> Vec<String> {
        self.console_output.clone()
    }
}

/// Выполняет JS-код и возвращает трассу событий Event Loop.
///
/// # Аргументы
/// - `source` — исходный код на JavaScript.
///
/// # Возвращает
/// Результат выполнения: события Event Loop и вывод консоли.
#[wasm_bindgen]
pub fn run(source: &str) -> RunResult {
    let mut engine = event_loop::EventLoopEngine::new();
    let result = engine.run(source);

    RunResult {
        events: result.events,
        console_output: result.console_output,
    }
}

/// Выполнить один шаг Event Loop (для пошагового режима визуализации).
///
/// # Аргументы
/// - `source`: исходный код.
/// - `step_index`: индекс шага, с которого продолжить.
#[wasm_bindgen]
pub fn step(source: &str, step_index: usize) -> RunResult {
    let mut engine = event_loop::EventLoopEngine::new();
    let result = engine.run_from_step(source, step_index);

    RunResult {
        events: result.events,
        console_output: result.console_output,
    }
}