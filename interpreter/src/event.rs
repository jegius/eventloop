//! Определение событий Event Loop.
//!
//! События генерируются интерпретатором и движком Event Loop для визуализации.
//! Вынесены в отдельный модуль, чтобы избежать циклической зависимости между
//! `interpreter` и `event_loop`.

use crate::renderer::RenderPhase;

/// Событие Event Loop (для визуализации).
#[derive(Debug, Clone, PartialEq)]
pub enum EventLoopEvent {
    /// Синхронный код выполнен.
    SyncCode(String),
    /// Пуш в Call Stack. Второй и третий аргументы — диапазон строк `(start, end)`
    /// в исходном коде (1-based, 0 — неизвестно). Диапазон покрывает весь колбэк.
    CallStackPush(String, usize, usize),
    /// Поп из Call Stack.
    CallStackPop(String),
    /// Добавление в Microtask Queue.
    MicrotaskEnqueue(String),
    /// Извлечение из Microtask Queue.
    MicrotaskDequeue(String),
    /// Добавление в Macrotask Queue.
    MacrotaskEnqueue(String),
    /// Извлечение из Macrotask Queue.
    MacrotaskDequeue(String),
    /// Добавление в rAF.
    RafEnqueue(String),
    /// Извлечение из rAF.
    RafDequeue(String),
    /// Добавление в rIC.
    RicEnqueue(String),
    /// Извлечение из rIC.
    RicDequeue(String),
    /// Запрос рендера.
    RenderRequested,
    /// Завершение рендера (удаление задачи рендера из очереди).
    RenderDequeue,
    /// Фаза рендера.
    RenderPhase(RenderPhase),
    /// Вывод в консоль.
    ConsoleLog(String),
    /// Предупреждение о потенциальном зацикливании очередей.
    ///
    /// Генерируется на этапе статического анализа AST (в парсере), когда
    /// обнаруживается код, который может привести к бесконечному зацикливанию
    /// микрозадач (рекурсивный `Promise.resolve().then(fn)`) или макрозадач
    /// (рекурсивный `setTimeout(fn, 0)`). Содержит текст предупреждения с
    /// рекомендацией прервать исполнение кнопкой сброса.
    Warning(String),
}