# Архитектура проекта

## Обзор

Проект состоит из двух независимых, но связанных частей:

1. **`interpreter/`** — Rust-интерпретатор JavaScript, компилируемый в WebAssembly.
2. **`frontend/`** — визуализатор на Web Components + TypeScript.

Связь между ними осуществляется через WASM-модуль: фронтенд загружает WASM,
передаёт в него JS-код, а интерпретатор возвращает трассировку выполнения
(события Event Loop) и результаты вывода в консоль.

---

## Интерпретатор (Rust → WASM)

### Слои

```
┌─────────────────────────────────────────────┐
│                WASM Boundary                │
│  (wasm-bindgen: run(), step(), getEvents()) │
├─────────────────────────────────────────────┤
│              Event Loop Engine              │
│  (event_loop.rs, queues.rs, renderer.rs)    │
├─────────────────────────────────────────────┤
│              Interpreter (AST)              │
│  (interpreter.rs, value.rs, environment.rs) │
├─────────────────────────────────────────────┤
│                   Parser                    │
│  (parser.rs, ast.rs)                        │
├─────────────────────────────────────────────┤
│                   Lexer                     │
│  (lexer.rs)                                 │
└─────────────────────────────────────────────┘
```

### Компоненты

| Модуль | Назначение |
|--------|-----------|
| `lexer.rs` | Преобразует исходный код в токены. |
| `parser.rs` | Строит AST из токенов (recursive descent). |
| `ast.rs` | Определения узлов AST. |
| `interpreter.rs` | Tree-walking интерпретатор. |
| `value.rs` | Представление значений JS (`Value`). |
| `environment.rs` | Лексические окружения, замыкания, `this`. |
| `event_loop.rs` | Реализация Event Loop по стандарту. |
| `queues.rs` | Очереди: microtask, macrotask, render, rAF, rIC. |
| `renderer.rs` | Минимальный рендер-движок (HTML/CSS/DOM/CSSOM). |
| `console.rs` | Сбор вывода в консоль. |

### Event Loop Engine

Ядро — `event_loop.rs`. Оно управляет очередями и фазами рендера. Каждый шаг
генерирует **событие** (Event), которое отправляется во фронтенд для визуализации.

```rust
// Пример структуры события
pub enum EventLoopEvent {
    CallStackPush(String),
    CallStackPop(String),
    MicrotaskEnqueue(String),
    MicrotaskDequeue(String),
    MacrotaskEnqueue(String),
    MacrotaskDequeue(String),
    RafEnqueue(String),
    RicEnqueue(String),
    RenderPhase(RenderPhase),
    ConsoleLog(String),
    // ...
}
```

### Renderer (минимальный рендер-движок)

Реализует настоящие (минимальные) фазы рендера:

1. **Parsing HTML** — разбор HTML в DOM.
2. **Parsing CSS** — разбор CSS в CSSOM.
3. **DOM** — построение дерева DOM.
4. **CSSOM** — построение дерева CSSOM.
5. **Render Tree** — объединение DOM + CSSOM.
6. **Layout** — вычисление геометрии.
7. **Paint** — отрисовка пикселей.
8. **Compose** — композиция слоёв.

Каждая фаза — реальная (не симуляция), но минимальная по объёму.

---

## 2. Фронтенд (Web Components + TypeScript)

### Компоненты

| Компонент | Назначение |
|-----------|-----------|
| `event-loop-visualizer` | Главный контейнер, оркестрация. |
| `call-stack` | Визуализация Call Stack. |
| `microtask-queue` | Визуализация Microtask Queue. |
| `macrotask-queue` | Визуализация Macrotask Queue. |
| `render-queue` | Визуализация Render Queue. |
| `render-phases` | Визуализация фаз рендера. |
| `code-editor` | Редактор JS-кода. |
| `console` | Консоль вывода. |

### Поток данных

```
User вводит код в <code-editor>
        │
        ▼
<event-loop-visualizer> вызывает WASM (interpreter)
        │
        ▼
WASM выполняет код, генерирует события EventLoopEvent[]
        │
        ▼
Фронтенд получает события и обновляет компоненты
        │
        ▼
<call-stack>, <microtask-queue>, ... обновляются
```

### Режимы работы

- **Пошаговый**: пользователь вручную переключает фазы.
- **Реальное время**: автоматическое выполнение с настраиваемой скоростью.

---

## 3. Сборка

### Интерпретатор

```bash
cd interpreter
wasm-pack build --target web
```

### Фронтенд

```bash
cd frontend
npm install
npm run dev
```

---

## 4. События и протокол

Интерпретатор возвращает массив событий `EventLoopEvent`. Фронтенд подписывается
на них и обновляет UI. Протокол событий описан в `interpreter/src/event_loop.rs`.

---

## 5. Тестирование

- **Rust**: юнит-тесты для лексера, парсера, интерпретатора, Event Loop.
- **TypeScript**: тесты компонентов (Vitest + Testing Library).

---

## 6. Расширяемость

- Добавление новых фаз рендера — расширение `renderer.rs`.
- Добавление новых очередей — расширение `queues.rs`.
- Добавление новых конструкций JS — расширение `parser.rs` и `interpreter.rs`.