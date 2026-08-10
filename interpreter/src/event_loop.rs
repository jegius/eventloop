//! Реализация Event Loop по стандарту ECMAScript 2026.
//!
//! Ядро механизма Event Loop. Управляет очередями (microtask, macrotask,
//! render, rAF, rIC), выполняет код и генерирует события для визуализации.

use crate::console::Console;
use crate::event::EventLoopEvent;
use crate::interpreter::Interpreter;
use crate::parser::Parser;
use crate::queues::{
    AnimationFrameQueue, IdleCallbackQueue, MacrotaskQueue, MicrotaskQueue, RenderQueue,
};
use crate::renderer::Renderer;

/// Результат выполнения Event Loop.
#[derive(Debug, Clone, Default)]
pub struct EventLoopResult {
    /// События Event Loop (в JSON-формате для WASM).
    pub events: Vec<String>,
    /// Вывод консоли.
    pub console_output: Vec<String>,
}

/// Движок Event Loop.
#[derive(Debug)]
pub struct EventLoopEngine {
    /// Очередь микрозадач.
    pub microtask_queue: MicrotaskQueue,
    /// Очередь макрозадач.
    pub macrotask_queue: MacrotaskQueue,
    /// Очередь rAF.
    pub raf_queue: AnimationFrameQueue,
    /// Очередь rIC.
    pub ric_queue: IdleCallbackQueue,
    /// Очередь рендера.
    pub render_queue: RenderQueue,
    /// Рендер-движок.
    pub renderer: Renderer,
    /// Консоль.
    pub console: Console,
    /// События.
    pub events: Vec<EventLoopEvent>,
}

impl EventLoopEngine {
    /// Создаёт новый движок Event Loop.
    pub fn new() -> Self {
        EventLoopEngine {
            microtask_queue: MicrotaskQueue::new(),
            macrotask_queue: MacrotaskQueue::new(),
            raf_queue: AnimationFrameQueue::new(),
            ric_queue: IdleCallbackQueue::new(),
            render_queue: RenderQueue::new(),
            renderer: Renderer::new(),
            console: Console::new(),
            events: Vec::new(),
        }
    }

    /// Выполняет JS-код и возвращает результат.
    pub fn run(&mut self, source: &str) -> EventLoopResult {
        self.events.clear();
        self.console.clear();

        // 1. Разбор кода
        let mut parser = match Parser::new(source) {
            Ok(p) => p,
            Err(e) => {
                self.console.error(&format!("Ошибка парсинга: {}", e));
                return self.finish();
            }
        };

        let program = match parser.parse_program() {
            Ok(p) => p,
            Err(e) => {
                self.console.error(&format!("Ошибка парсинга: {}", e));
                return self.finish();
            }
        };

        // 2. Статический анализ: детекция потенциального зацикливания очередей.
        //    Предупреждения добавляются в начало списка событий, чтобы
        //    визуализатор показал их до начала исполнения.
        for warning in parser.detect_queue_loops(&program) {
            self.events.push(EventLoopEvent::Warning(warning));
        }

        // 3. Событие синхронного кода
        self.events.push(EventLoopEvent::SyncCode(
            format!("Разобрано {} операторов", program.statements.len()),
        ));

        // 3. Выполнение синхронного кода через интерпретатор
        // Каждый оператор попадает в call stack отдельно (генерируется в interpret).
        let mut interpreter = Interpreter::new();
        let _ = interpreter.interpret(&program);
        // Вставляем события консоли (console.log) в правильном порядке
        self.events.extend(interpreter.events.drain(..));

        // 4. Обработка очередей (остановка вывода консоли выполняется внутри)
        //
        // Рендер запрашивается только если есть что рендерить: rAF/rIC колбэки
        // или синхронный код, который мог изменить DOM. Это соответствует поведению
        // браузера, где рендер выполняется не всегда, а только при необходимости.
        if !interpreter.raf_queue.borrow().is_empty()
            || !interpreter.ric_queue.borrow().is_empty()
        {
            interpreter.render_queue.borrow_mut().request_render();
        }
        self.process_queues(&mut interpreter);

        self.finish()
    }

    /// Выполняет код, начиная с указанного шага (для пошагового режима).
    pub fn run_from_step(&mut self, source: &str, _step_index: usize) -> EventLoopResult {
        // Заглушка: на этапе 2 будет реализован пошаговый режим.
        self.run(source)
    }

    /// Обрабатывает очереди Event Loop.
    ///
    /// Порядок обработки соответствует спецификации браузера (WHATWG HTML):
    /// 1. Выполняется ОДНА самая старая макрозадача (task).
    /// 2. Microtask checkpoint — выполняются ВСЕ микрозадачи до конца.
    /// 3. Update the rendering — если наступило время кадра и есть что рендерить:
    ///    - requestAnimationFrame (rAF) колбэки;
    ///    - фазы рендера (style, layout, paint, composite);
    ///    - requestIdleCallback (rIC) — в свободное время.
    /// 4. Цикл повторяется.
    ///
    /// В отличие от упрощённой модели, рендер происходит ПОСЛЕ каждой макрозадачи
    /// (в конце цикла), а не после всей очереди макрозадач. Кроме того, рендер
    /// выполняется НЕ всегда и НЕ полностью: он может быть пропущен (нет времени
    /// кадра / нет изменений) или выполнен частично (не все фазы).
    fn process_queues(&mut self, interpreter: &mut Interpreter) {
        // Счётчик повторных setInterval (ограничиваем, чтобы избежать бесконечного цикла).
        let mut interval_count = 0;
        // Счётчик обработанных макрозадач. Ограничиваем общее число макрозадач,
        // чтобы гарантированно не зависнуть на рекурсивном setTimeout (когда
        // статический анализ не сработал или код добавляет макрозадачи динамически).
        let mut macrotask_count = 0;
        // Максимальное число макрозадач за всё выполнение.
        const MAX_MACROTASKS: usize = 100;

        // 0. Microtask checkpoint — выполняем микрозадачи, добавленные синхронным
        //    кодом, ДО обработки первой макрозадачи. Это соответствует поведению
        //    браузера: после выполнения скрипта (синхронного кода) выполняется
        //    microtask checkpoint, и только потом обрабатываются макрозадачи.
        self.process_microtasks(interpreter);

        // Основной цикл: обрабатываем макрозадачи по одной.
        //
        // Порядок в каждой итерации соответствует браузеру (WHATWG HTML):
        //   1. Update the rendering (rAF + фазы рендера) — выполняется ДО обработки
        //      макрозадач. В браузере после завершения синхронного кода и microtask
        //      checkpoint браузер отрисовывает кадр (выполняя rAF колбэки), и только
        //      потом переходит к обработке макрозадач.
        //   2. Одна самая старая макрозадача.
        //   3. Microtask checkpoint — все микрозадачи до конца.
        //   4. Idle period — requestIdleCallback (rIC) выполняется в свободное время
        //      ПОСЛЕ обработки макрозадач.
        loop {
            // 1. Update the rendering (rAF + фазы рендера) — перед макрозадачей.
            self.process_raf_and_render(interpreter);

            // 2. Извлекаем одну самую старую макрозадачу.
            let task = interpreter.macrotask_queue.borrow_mut().dequeue();
            let Some(task) = task else { break; };
            macrotask_count += 1;
            self.events.push(EventLoopEvent::MacrotaskDequeue(task.label.clone()));
            let (start, end) = task.callback_line_range();
            self.events.push(EventLoopEvent::CallStackPush(task.label.clone(), start, end));
            let globals = interpreter.globals.clone();
            let _ = interpreter.call_function(&task.callback, &task.args, &globals);
            self.events.extend(interpreter.events.drain(..));
            self.events.push(EventLoopEvent::CallStackPop(task.label.clone()));

            // setInterval выполняется повторно, но ограниченное число раз (3), чтобы не зациклиться.
            if task.label == "setInterval" && interval_count < 3 {
                interval_count += 1;
                interpreter
                    .macrotask_queue
                    .borrow_mut()
                    .enqueue("setInterval", task.callback.clone(), task.args.clone());
                self.events.push(EventLoopEvent::MacrotaskEnqueue("setInterval".to_string()));
            }

            // 3. Microtask checkpoint — выполняем все микрозадачи до конца.
            self.process_microtasks(interpreter);

            // 4. Idle period — rIC выполняется после макрозадачи.
            self.process_ric(interpreter);

            // Защита от бесконечного зацикливания макрозадач (рекурсивный setTimeout).
            if macrotask_count >= MAX_MACROTASKS {
                break;
            }
        }

        // После обработки всех макрозадач выполняем финальный microtask checkpoint,
        // шаг рендеринга и idle period (как в браузере после завершения всех задач).
        self.process_microtasks(interpreter);
        self.process_raf_and_render(interpreter);
        self.process_ric(interpreter);

        // Копируем вывод консоли из интерпретатора
        for line in interpreter.console.get_output() {
            self.console.log(&line);
        }
    }

    /// Выполняет все микрозадачи до конца (microtask checkpoint).
    ///
    /// Вновь добавленные микрозадачи также выполняются в рамках этого шага,
    /// как это происходит в браузере.
    ///
    /// Защита от бесконечного зацикливания микрозадач (например, рекурсивного
    /// `Promise.resolve().then(fn)`) выполняется на этапе статического анализа
    /// AST (см. [`Parser::detect_queue_loops`]), который выводит предупреждение
    /// с рекомендацией прервать исполнение кнопкой сброса.
    ///
    /// Дополнительно, чтобы гарантированно не зависнуть на рекурсивной
    /// микрозадаче (когда статический анализ не сработал или код добавляет
    /// микрозадачи динамически), вводится жёсткий лимит на количество
    /// микрозадач, выполняемых за один checkpoint. Лимит достаточно велик,
    /// чтобы не прерывать легитимный код с большим количеством промисов
    /// (например, цепочку из 40 промисов), но предотвращает бесконечное
    /// зацикливание.
    fn process_microtasks(&mut self, interpreter: &mut Interpreter) {
        // Максимальное число микрозадач за один checkpoint.
        const MAX_MICROTASKS_PER_CHECKPOINT: usize = 100;
        let mut processed = 0;

        while !interpreter.microtask_queue.borrow().is_empty() && processed < MAX_MICROTASKS_PER_CHECKPOINT {
            let task = interpreter.microtask_queue.borrow_mut().dequeue();
            if let Some(task) = task {
                processed += 1;
                self.events.push(EventLoopEvent::MicrotaskDequeue(task.label.clone()));
                let (start, end) = task.callback_line_range();
            self.events.push(EventLoopEvent::CallStackPush(task.label.clone(), start, end));
                // Выполняем колбэк
                let globals = interpreter.globals.clone();
                let result = interpreter.call_function(&task.callback, &task.args, &globals);
                self.events.extend(interpreter.events.drain(..));
                self.events.push(EventLoopEvent::CallStackPop(task.label.clone()));

                // Обработка цепочки промисов: если колбэк — часть `.then().then()`,
                // `task.next_promise` — новый Promise. Разрешаем его результатом колбэка
                // и добавляем его `then_callbacks` в microtask queue (batching микрозадач).
                // Внутренний Promise не передаётся пользовательскому колбэку как аргумент.
                if let Ok(result_value) = result {
                    if let Some(crate::value::Value::Promise(next_promise)) = task.next_promise {
                        next_promise.borrow_mut().state = crate::value::PromiseState::Fulfilled;
                        next_promise.borrow_mut().value = Some(Box::new(result_value.clone()));
                        // Добавляем then_callbacks нового Promise в microtask queue.
                        let callbacks = next_promise.borrow().then_callbacks.clone();
                        for cb in callbacks {
                            let label = cb.display();
                            interpreter
                                .microtask_queue
                                .borrow_mut()
                                .enqueue(&label, cb, vec![result_value.clone()]);
                            self.events
                                .push(EventLoopEvent::MicrotaskEnqueue(label));
                        }
                    }
                }
            }
        }
    }

    /// Выполняет шаг рендеринга (update the rendering) — rAF колбэки и фазы рендера.
    ///
    /// Этот шаг выполняется ДО обработки макрозадач, как в браузере: после
    /// завершения синхронного кода и microtask checkpoint браузер отрисовывает
    /// кадр (выполняя rAF колбэки), и только потом переходит к макрозадачам.
    ///
    /// В отличие от упрощённой модели, рендер выполняется НЕ всегда:
    /// - если нет запроса на рендер (render_queue.needs_render() == false),
    ///   шаг пропускается;
    /// - если рендер выполняется, он может быть частичным (не все фазы
    ///   выполняются, если нет соответствующих изменений).
    ///
    /// В рамках шага рендеринга выполняются:
    /// 1. requestAnimationFrame (rAF) колбэки;
    /// 2. фазы рендера (style → layout → paint → composite).
    fn process_raf_and_render(&mut self, interpreter: &mut Interpreter) {
        // Рендер выполняется только если он был запрошен (есть изменения).
        // Это соответствует поведению браузера: если DOM не менялся и не наступило
        // время кадра, рендеринг не происходит.
        if !interpreter.render_queue.borrow().needs_render() {
            // Сбрасываем флаг на всякий случай.
            interpreter.render_queue.borrow_mut().reset();
            return;
        }

        // 1. requestAnimationFrame — колбэки выполняются в начале шага рендеринга.
        if !interpreter.raf_queue.borrow().is_empty() {
            self.events.push(EventLoopEvent::RenderPhase(crate::renderer::RenderPhase::AnimationFrame));
            while !interpreter.raf_queue.borrow().is_empty() {
                let task = interpreter.raf_queue.borrow_mut().dequeue();
                if let Some(task) = task {
                    self.events.push(EventLoopEvent::RafDequeue(task.label.clone()));
                    let (start, end) = task.callback_line_range();
            self.events.push(EventLoopEvent::CallStackPush(task.label.clone(), start, end));
                    let globals = interpreter.globals.clone();
                    let _ = interpreter.call_function(&task.callback, &task.args, &globals);
                    self.events.extend(interpreter.events.drain(..));
                    self.events.push(EventLoopEvent::CallStackPop(task.label.clone()));
                }
            }
        }

        // 2. Фазы рендера. Рендер может быть частичным: если DOM не менялся,
        //    фазы Layout и Paint могут быть пропущены. Для простоты визуализации
        //    выполняем полный набор фаз, но помечаем, что рендер запрошен.
        //
        //    Рендер-движок разбирает демонстрационный HTML/CSS, чтобы показать
        //    все фазы: Parsing HTML → Parsing CSS → DOM → CSSOM → Render Tree
        //    → Layout → Paint → Compose.
        self.events.push(EventLoopEvent::RenderRequested);
        let demo_html = "<html><body><div class='box'>Hello</div></body></html>";
        let demo_css = "body { margin: 0; } .box { width: 100px; height: 50px; color: red; }";
        let phases = self.renderer.render(demo_html, demo_css);
        for phase in phases {
            self.events.push(EventLoopEvent::RenderPhase(phase));
        }

        // Сбрасываем флаг рендера после выполнения шага.
        interpreter.render_queue.borrow_mut().reset();
    }

    /// Выполняет requestIdleCallback (rIC) колбэки в свободное время (idle period).
    ///
    /// В браузере rIC выполняется ПОСЛЕ обработки макрозадач, когда браузер
    /// простаивает. Поэтому этот шаг вызывается после обработки каждой макрозадачи
    /// и в конце цикла, а не в рамках шага рендеринга.
    fn process_ric(&mut self, interpreter: &mut Interpreter) {
        loop {
            let task = interpreter.ric_queue.borrow_mut().dequeue();
            let Some(task) = task else { break; };
            self.events.push(EventLoopEvent::RicDequeue(task.label.clone()));
            let (start, end) = task.callback_line_range();
            self.events.push(EventLoopEvent::CallStackPush(task.label.clone(), start, end));
            let globals = interpreter.globals.clone();
            let _ = interpreter.call_function(&task.callback, &task.args, &globals);
            self.events.extend(interpreter.events.drain(..));
            self.events.push(EventLoopEvent::CallStackPop(task.label.clone()));
        }
    }

    /// Завершает выполнение и возвращает результат.
    fn finish(&mut self) -> EventLoopResult {
        EventLoopResult {
            events: self.events.iter().map(|e| format!("{:?}", e)).collect(),
            console_output: self.console.get_output(),
        }
    }
}

impl Default for EventLoopEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_basic() {
        let mut engine = EventLoopEngine::new();
        let result = engine.run("let x = 42;");
        assert!(!result.events.is_empty());
    }

    #[test]
    fn test_run_parse_error() {
        let mut engine = EventLoopEngine::new();
        let result = engine.run("let = ;");
        assert!(!result.console_output.is_empty());
    }

    #[test]
    fn test_render_phases_generated() {
        // Код с requestAnimationFrame должен инициировать рендер.
        let mut engine = EventLoopEngine::new();
        let result = engine.run("requestAnimationFrame(() => {});");
        let events = result.events.join("\n");
        // Проверяем, что фазы рендера генерируются.
        assert!(events.contains("RenderRequested"), "ожидался RenderRequested");
        assert!(events.contains("RenderPhase(ParsingHtml)"), "ожидался ParsingHtml");
        assert!(events.contains("RenderPhase(Layout)"), "ожидался Layout");
        assert!(events.contains("RenderPhase(Compose)"), "ожидался Compose");
    }

    #[test]
    fn test_promise_resolve_then() {
        // Promise.resolve(4).then(console.log) должен вывести 4.
        let mut engine = EventLoopEngine::new();
        let result = engine.run("Promise.resolve(4).then(console.log);");
        let output = result.console_output.join(" ");
        assert_eq!(output, "4", "неверный вывод: {}", output);
    }

    #[test]
    fn test_microtasks_before_macrotasks() {
        // Микрозадачи (Promise) должны выполняться ДО макрозадач (setTimeout).
        let mut engine = EventLoopEngine::new();
        let result = engine.run(
            "console.log(1); \
             Promise.resolve(4).then(console.log); \
             setTimeout(() => console.log(5)); \
             console.log(6); \
             new Promise((resolve) => { console.log(2); resolve(3); }).then(console.log); \
             setTimeout(() => console.log(7), 0);",
        );
        // Ожидаемый порядок: 1 6 2 4 3 5 7
        let output = result.console_output.join(" ");
        assert_eq!(output, "1 6 2 4 3 5 7", "неверный порядок выполнения: {}", output);
    }

    #[test]
    fn test_promise_chain_batching() {
        // Цепочка промисов: вторая .then должна получить результат первой .then.
        let mut engine = EventLoopEngine::new();
        let result = engine.run(
            "new Promise((resolve) => { resolve(3); }) \
             .then((result) => { console.log(result); return 8; }) \
             .then(console.log);",
        );
        // Ожидаемый порядок: 3 8
        let output = result.console_output.join(" ");
        assert_eq!(output, "3 8", "неверный порядок выполнения: {}", output);
    }

    #[test]
    fn test_microtask_loop_warning() {
        // Рекурсивная микрозадача: someFuncOne вызывает Promise.resolve().then(someFuncOne).
        // Статический анализ должен обнаружить потенциальное зацикливание микрозадач
        // и сгенерировать событие Warning с рекомендацией прервать исполнение.
        let mut engine = EventLoopEngine::new();
        let result = engine.run(
            "const someFuncOne = () => { \
                Promise.resolve().then(someFuncOne); \
             }; \
             someFuncOne();",
        );
        // Проверяем, что сгенерировано событие Warning.
        let events = result.events.join("\n");
        assert!(
            events.contains("Warning"),
            "ожидалось событие Warning, но события: {}",
            events
        );
        assert!(
            events.contains("зацикливанию микрозадач"),
            "ожидалось предупреждение о зацикливании микрозадач, но события: {}",
            events
        );
        assert!(
            events.contains("Сброс"),
            "ожидалась рекомендация прервать исполнение кнопкой «Сброс», но события: {}",
            events
        );
    }

    #[test]
    fn test_macrotask_loop_warning() {
        // Рекурсивная макрозадача: someFuncTwo вызывает setTimeout(someFuncTwo, 0).
        // Статический анализ должен обнаружить потенциальное зацикливание макрозадач
        // и сгенерировать событие Warning с рекомендацией прервать исполнение.
        let mut engine = EventLoopEngine::new();
        let result = engine.run(
            "const someFuncTwo = () => { \
                setTimeout(someFuncTwo, 0); \
             }; \
             someFuncTwo();",
        );
        // Проверяем, что сгенерировано событие Warning.
        let events = result.events.join("\n");
        assert!(
            events.contains("Warning"),
            "ожидалось событие Warning, но события: {}",
            events
        );
        assert!(
            events.contains("зацикливанию макрозадач"),
            "ожидалось предупреждение о зацикливании макрозадач, но события: {}",
            events
        );
        assert!(
            events.contains("Сброс"),
            "ожидалась рекомендация прервать исполнение кнопкой «Сброс», но события: {}",
            events
        );
    }

    #[test]
    fn test_no_false_positive_for_many_promises() {
        // Легитимный код с большим количеством промисов (не рекурсивных) НЕ должен
        // генерировать предупреждение о зацикливании (в отличие от старого подхода
        // с жёстким лимитом в 30 задач).
        let mut engine = EventLoopEngine::new();
        let result = engine.run(
            "let p = Promise.resolve(); \
             for (let i = 0; i < 40; i++) { p = p.then(() => {}); }",
        );
        let events = result.events.join("\n");
        assert!(
            !events.contains("Warning"),
            "не ожидалось предупреждение для цепочки из 40 промисов, но события: {}",
            events
        );
    }
}