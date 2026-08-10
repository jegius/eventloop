# Custom JS Event Loop — визуализатор механизма Event Loop

Интерактивный инструмент для глубокого изучения механизма работы **Event Loop** в JavaScript
согласно современному стандарту **ECMAScript 2026**. Проект состоит из двух частей:

1. **Интерпретатор JavaScript** (на **Rust**, компилируется в **WebAssembly**) — настоящий,
   готовый к самостоятельному использованию интерпретатор, реализующий механизм Event Loop
   по стандарту ECMAScript 2026.
2. **Визуализатор** (на **Web Components + TypeScript**) — инструмент, позволяющий гибко
   «препарировать» работу Event Loop: call stack, microtask queue, macrotask queue,
   render queue, requestAnimationFrame, requestIdleCallback, а также все фазы рендера
   (parsing HTML, parsing CSS, DOM, CSSOM, render tree, layout, paint, compose).

---

## 🎯 Цели проекта

- Глубокое изучение механизма Event Loop в браузере.
- Реальная (не симуляция) реализация минимального подмножества ECMAScript 2026,
  достаточного для демонстрации всех аспектов Event Loop.
- Наглядная визуализация каждой фазы Event Loop и каждой фазы рендера.
- Возможность вставлять произвольный JS-код (в рамках поддерживаемого подмножества),
  наблюдать, как он триггерит очереди, и как он выполняется в реальном интерпретаторе.
- Встроенная консоль вывода результатов.

---

## 🏗 Архитектура

Проект построен как **монорепо** с двумя основными пакетами:

```
custom-js-eventloop/
├── README.md                  # Этот файл
├── docs/
│   ├── architecture.md        # Подробная архитектура
│   ├── roadmap.md              # План реализации
│   └── rules.md                # Правила разработки
├── interpreter/                # Rust-интерпретатор → WASM
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs              # Точка входа, экспорт в WASM (run/step)
│   │   ├── lexer.rs            # Лексический анализатор
│   │   ├── parser.rs           # Синтаксический анализатор (AST) + статический анализ
│   │   ├── ast.rs              # Определения AST
│   │   ├── interpreter.rs      # Интерпретатор (tree-walking) + встроенные функции
│   │   ├── value.rs            # Значения JS (Value, Object, Function, Promise)
│   │   ├── environment.rs      # Окружения (lexical environments)
│   │   ├── event_loop.rs       # Реализация Event Loop Engine
│   │   ├── event.rs            # События Event Loop (EventLoopEvent)
│   │   ├── queues.rs           # Очереди (micro/macro/render/rAF/rIC)
│   │   ├── renderer.rs         # Минимальный рендер-движок (HTML/CSS/DOM/CSSOM)
│   │   └── console.rs          # Консоль вывода
│   └── tests/
├── frontend/                   # Web Components + TypeScript
│   ├── package.json
│   ├── tsconfig.json
│   ├── vite.config.ts
│   ├── index.html
│   └── src/
│       ├── main.ts             # Точка входа, регистрация компонентов
│       ├── types.ts            # Типы событий Event Loop
│       ├── di/container.ts     # DI-контейнер (Service Locator)
│       ├── services/           # Сервисы (паттерн Service)
│       │   ├── register.ts     # Регистрация сервисов в DI
│       │   ├── tokens.ts       # Токены сервисов
│       │   ├── interpreter.service.ts  # Обёртка над WASM
│       │   ├── event-parser.service.ts # Парсинг событий
│       │   └── playback.service.ts     # Воспроизведение событий
│       ├── components/         # Web Components (MVC: .ts/.controller/.template/.styles)
│       │   ├── event-loop-visualizer/  # Главный визуализатор (оркестратор)
│       │   ├── call-stack/              # Call Stack
│       │   ├── queue/                   # Универсальная очередь (micro/macro/render)
│       │   ├── render-phases/           # Фазы рендера
│       │   ├── code-editor/             # Редактор кода
│       │   ├── console/                 # Консоль
│       │   └── execution-indicator/     # Индикатор потока исполнения
│       ├── wasm/                        # Загрузка WASM-модуля
│       └── styles/
└── .idea/                     # Конфигурация IDEA
```

### Поток данных (фронтенд)

```
Пользователь вводит код в <code-editor>
        │  (событие 'run-code')
        ▼
<event-loop-visualizer> → EventLoopVisualizerController.executeCode()
        │  (InterpreterService.run → WASM)
        ▼
WASM-интерпретатор выполняет код, генерирует события EventLoopEvent[]
        │  (строковые события)
        ▼
PlaybackService.loadEvents() → EventParserService.parseAll()
        │  (типизированные события)
        ▼
PlaybackService (realtime/step) → onEvent → Controller.processEvent()
        │
        ▼
<call-stack>, <queue>, <render-phases>, <console>, <execution-indicator> обновляются
```

### Слои интерпретатора

```
┌─────────────────────────────────────────────┐
│                WASM Boundary                │
│  (lib.rs: run(), step())                    │
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

---

## 🧩 Технологический стек

| Слой | Технология | Обоснование |
|------|-----------|-------------|
| **Интерпретатор** | **Rust** → WebAssembly | Высокая производительность, отличная интеграция с WASM (`wasm-bindgen`), безопасность памяти, малый runtime-оверхед |
| **Фронтенд** | **Web Components + TypeScript** | Нативные веб-компоненты, изолированные, переиспользуемые, без тяжёлых фреймворков |
| **Сборка фронта** | **Vite** | Быстрая сборка, HMR, отличная поддержка WASM |
| **Сборка WASM** | **wasm-pack / wasm-bindgen** | Стандартный инструментарий для Rust → WASM |

---

## 🎯 Область ECMAScript 2026 (подмножество для MVP)

Реализуется только та часть стандарта, которая необходима для глубокого изучения
механизма Event Loop:

- **Базовые типы**: `undefined`, `null`, `boolean`, `number`, `string`, `object`, `array`, `function`.
- **Операторы**: арифметические, сравнения, логические, присваивание, тернарный.
- **Управляющие конструкции**: `if/else`, `for`, `while`, `switch`, `break`, `continue`.
- **Функции**: объявления, выражения, стрелочные функции, замыкания, `this`.
- **Объекты и массивы**: литералы, доступ к свойствам, методы.
- **Промисы**: `Promise`, `resolve`, `reject`, `then`, `catch`, `finally`.
- **Async/await**: `async` функции, `await`.
- **Классы**: `class`, конструкторы, методы, наследование.
- **Таймеры**: `setTimeout`, `setInterval`, `setImmediate` (для демонстрации macrotask).
- **Очереди**: `queueMicrotask`, `requestAnimationFrame`, `requestIdleCallback`.
- **Модули** (опционально, на поздних этапах).

> ⚠️ Полная реализация ECMAScript — это многолетний проект (V8, SpiderMonkey, JSC
> разрабатываются десятилетиями). Мы реализуем **функциональное подмножество**, достаточное
> для демонстрации и изучения Event Loop.

---

## 🔄 Механизм Event Loop (что визуализируем)

Согласно стандарту ECMAScript 2026 и HTML Living Standard, Event Loop работает так:

```
┌─────────────────────────────────────────────────────────────┐
│                    EVENT LOOP (браузер)                     │
│                                                             │
│  ┌─────────────┐   ┌──────────────────┐   ┌──────────────┐  │
│  │  Call Stack │   │  Microtask Queue │   │ Macrotask Q  │  │
│  │  (стек)     │   │  (Promise, etc)  │   │ (setTimeout) │  │
│  └─────────────┘   └──────────────────┘   └──────────────┘  │
│                                                             │
│  ┌──────────────────────────────────────────────────────┐   │
│  │              Render Steps (каждый кадр)              │   │
│  │  Parsing HTML → Parsing CSS → DOM → CSSOM →          │   │
│  │  Render Tree → Layout → Paint → Compose              │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                             │
│  ┌──────────────────┐   ┌──────────────────────┐            │
│  │ requestAnimation │   │ requestIdleCallback  │            │
│  │ Frame (rAF)      │   │ (rIC)                │            │
│  └──────────────────┘   └──────────────────────┘            │
└─────────────────────────────────────────────────────────────┘
```

### Порядок выполнения (упрощённо)

1. Выполняется синхронный код (заполняет Call Stack).
2. Когда стек пуст — обрабатывается **Microtask Queue** (Promise, queueMicrotask)
   до полного опустошения.
3. Затем обрабатывается **Macrotask Queue** (setTimeout, setInterval, I/O).
4. Перед рендером выполняются **requestAnimationFrame** колбэки.
5. Выполняется **рендер** (Render Steps).
6. В свободное время выполняются **requestIdleCallback** колбэки.
7. Цикл повторяется.

---

## 🖥 Визуализация

Визуализатор позволяет:

- **Вставлять JS-код** в редактор (в рамках поддерживаемого подмножества ECMAScript).
- **Наблюдать** как код триггерит очереди и выполняется в реальном интерпретаторе.
- **Пошаговый режим**: ручное переключение между фазами Event Loop (кнопка «Шаг»).
- **Режим реального времени**: автоматическое выполнение с настраиваемой скоростью
  (ползунок скорости, мс на шаг).
- **Визуализация очередей**: call stack, microtask queue, macrotask queue, render queue,
  rAF, rIC — с анимацией добавления/удаления элементов.
- **Визуализация фаз рендера**: parsing HTML, parsing CSS, DOM, CSSOM, render tree,
  layout, paint, compose — каждая фаза подсвечивается.
- **Индикатор потока исполнения**: показывает, где в данный момент находится
  исполнение (call stack / microtask / macrotask / render / idle).
- **Подсветка кода**: активный колбэк подсвечивается в редакторе (диапазон строк).
- **Консоль**: реальная консоль вывода результатов выполнения кода.
- **Журнал событий**: лог всех событий Event Loop в JSON-формате.
- **Предупреждения**: статический анализ обнаруживает потенциальное зацикливание
  очередей (рекурсивный `Promise.resolve().then(fn)` или `setTimeout(fn, 0)`) и
  выводит предупреждение с рекомендацией прервать исполнение кнопкой «Сброс».

### Примеры кода для демонстрации

**Порядок microtask/macrotask:**
```js
console.log(1);
Promise.resolve(4).then(console.log);
setTimeout(() => console.log(5));
console.log(6);
new Promise((resolve) => { console.log(2); resolve(3); }).then(console.log);
setTimeout(() => console.log(7), 0);
// Ожидаемый вывод: 1 6 2 4 3 5 7
```

**Цепочка промисов (batching микрозадач):**
```js
new Promise((resolve) => { resolve(3); })
  .then((result) => { console.log(result); return 8; })
  .then(console.log);
// Ожидаемый вывод: 3 8
```

**Async/await:**
```js
async function greet() {
  const value = await Promise.resolve('Привет');
  console.log(value);
}
greet();
```

**Таймеры и очереди:**
```js
setTimeout(() => console.log('setTimeout'));
setImmediate(() => console.log('setImmediate'));
queueMicrotask(() => console.log('queueMicrotask'));
requestAnimationFrame(() => console.log('rAF'));
requestIdleCallback(() => console.log('rIC'));
```

**Предупреждение о зацикливании микрозадач:**
```js
const someFuncOne = () => {
  Promise.resolve().then(someFuncOne);
};
someFuncOne();
// Статический анализ выведет предупреждение о потенциальном зацикливании.
```

---

## 📚 Правила разработки

См. [`docs/rules.md`](docs/rules.md) и [`docs/architecture.md`](docs/architecture.md).

---

## 🚀 Быстрый старт

### Требования

- **Node.js** ≥ 18 (проверено на v22)
- **Rust** (для компиляции интерпретатора в WASM) — `cargo`, `wasm-pack`
- **npm**

### Установка и запуск фронтенда

```bash
cd frontend
npm install
npm run dev
```

Откройте `http://localhost:5173` в браузере.

### Сборка интерпретатора в WASM

> ⚠️ Требуется Rust и `wasm-pack`. WASM-цель `wasm32-unknown-unknown` должна быть установлена.

```bash
# Установка WASM-цели (если используется rustup)
rustup target add wasm32-unknown-unknown

cd interpreter
wasm-pack build --target web
```

После сборки скопируйте сгенерированные файлы в `frontend/src/wasm/`:

```bash
cp interpreter/pkg/js_eventloop_interpreter.js frontend/src/wasm/
cp interpreter/pkg/js_eventloop_interpreter_bg.wasm frontend/src/wasm/
cp interpreter/pkg/js_eventloop_interpreter.d.ts frontend/src/wasm/
cp interpreter/pkg/js_eventloop_interpreter_bg.wasm.d.ts frontend/src/wasm/
```

### Сборка для продакшена

```bash
cd frontend
npm run build
```

---

## 🌐 Развёртывание на GitHub Pages

Проект автоматически разворачивается на **GitHub Pages** с помощью GitHub Actions
(см. [`.github/workflows/deploy.yml`](.github/workflows/deploy.yml)).

### Как это работает

1. При пуше в ветку `main` (или вручную через вкладку **Actions**) запускается workflow.
2. Workflow устанавливает зависимости и собирает фронтенд (`npm run build`).
3. Собранная папка `frontend/dist` публикуется на GitHub Pages.

### Первоначальная настройка (один раз)

1. **Создайте репозиторий** на GitHub и запушьте в него проект (ветка `main`).
2. В репозитории откройте **Settings → Pages**.
3. В разделе **Build and deployment** выберите **Source: GitHub Actions**.
4. Запушьте изменения в `main` — workflow запустится автоматически и опубликует сайт.

> ⚠️ **Важно:** скомпилированный WASM-модуль
> (`frontend/src/wasm/js_eventloop_interpreter_bg.wasm`) должен быть закоммичен в
> репозиторий — CI собирает только фронтенд и не пересобирает Rust-интерпретатор.
> Файл [`frontend/src/wasm/.gitignore`](frontend/src/wasm/.gitignore) уже настроен
> соответствующим образом.

### Адрес сайта

После деплоя сайт будет доступен по адресу:

```
https://<username>.github.io/<repository-name>/
```

Например, для пользователя `alex` и репозитория `custom-js-eventloop`:
`https://alex.github.io/custom-js-eventloop/`.

> В [`frontend/vite.config.ts`](frontend/vite.config.ts) задан относительный `base: './'`,
> поэтому сборка корректно работает в подпапке репозитория.

### Ручной запуск деплоя

1. Откройте вкладку **Actions** в репозитории.
2. Выберите workflow **Deploy to GitHub Pages**.
3. Нажмите **Run workflow**.

### Запуск тестов

```bash
# Тесты Rust-интерпретатора (лексер, парсер, интерпретатор, Event Loop)
cd interpreter
cargo test

# Тесты TypeScript-компонентов
cd frontend
npm run test
```

---

## 🧪 Использование

1. **Запустите фронтенд** (`npm run dev` в `frontend/`) и откройте `http://localhost:5173`.
2. **Вставьте JS-код** в редактор (см. примеры выше).
3. **Нажмите «Запуск»** — код выполнится в реальном WASM-интерпретаторе.
4. **Наблюдайте** за движением индикатора потока исполнения, очередями, фазами рендера
   и выводом в консоль.
5. **Переключите режим** на «Пошаговый» и нажимайте «Шаг», чтобы вручную проходить
   каждое событие Event Loop.
6. **Настройте скорость** ползунком (мс на шаг) в режиме реального времени.
7. **Сбросьте** состояние кнопкой «Сброс» при необходимости (например, при
   зацикливании очередей).

---

## ✅ Текущее состояние

Проект находится на **этапе 8 дорожной карты** (все этапы завершены). Реализовано:

- **Rust-интерпретатор** — лексер, парсер, tree-walking интерпретатор, Event Loop Engine,
  очереди (micro/macro/render/rAF/rIC), минимальный рендер-движок (8 фаз), консоль.
  Все юнит-тесты проходят.
- **WASM-модуль** — интерпретатор скомпилирован в WebAssembly и подключён к фронтенду.
- **Визуализатор** — Web Components: call stack, очереди, render phases, консоль, event log.
  Режимы: реальное время и пошаговый. Все тесты компонентов проходят.
- **Исправление Promise** — внутренний Promise для batching цепочек `.then().then()`
  хранится в отдельном поле `next_promise` задачи и не передаётся пользовательскому
  колбэку как лишний аргумент (например, `console.log`).

**Поддерживаемые конструкции (Этапы 1–2):**
- Переменные: `let`, `const`, `var`.
- Управляющие конструкции: `if/else`, `for`, `while`, `switch`, `break`, `continue`, `try/catch/finally`, `throw`.
- Функции: объявления, выражения, стрелочные (`=>`), `this`, `async`/`await`.
- Объекты и массивы: литералы, доступ к свойствам, методы.
- Операторы: арифметические, сравнения (`==`, `===`, `!=`, `!==` с корректным приведением типов), логические, тернарный, инкремент/декремент (`++`, `--`).
- Промисы: `Promise`, `resolve`, `then`, `catch`, `finally`.
- Таймеры и очереди: `setTimeout`, `setInterval`, `setImmediate`, `queueMicrotask`, `requestAnimationFrame`, `requestIdleCallback`.
- Браузерные API: `console.log`, `document`/`window` события, `fetch`.

**Ограничения текущей реализации:**
- Рендер-движок выполняет все 8 фаз, но с минимальной (заглушечной) реализацией
  парсинга HTML/CSS.
- Наследование классов (`extends`) парсится, но не полностью реализовано в интерпретаторе.
- `for...of` / `for...in` и `do...while` пока не реализованы.

---

## 📐 Соответствие спецификации ECMAScript

Интерпретатор реализует **функциональное подмножество** ECMAScript, достаточное для
демонстрации и изучения Event Loop. Ниже — детальная оценка соответствия текущей
реализации спецификации ECMAScript (по состоянию на текущий код).

### Что соответствует спецификации ✅

| Аспект | Оценка | Комментарий |
|--------|--------|-------------|
| **Числа** | ✅ | IEEE 754 double (`f64`), как в JS. |
| **Строгое сравнение `===` / `!==`** | ✅ | Без приведения типов; объекты сравниваются по ссылке (`std::ptr::eq` / `Rc::ptr_eq`). |
| **Нестрогое сравнение `==` / `!=`** | ✅ | Реализовано приведение типов: `null == undefined`, boolean→number, string→number, объект→примитив. |
| **Truthy/falsy** | ✅ | `0`, `NaN`, `""`, `null`, `undefined` — falsy; остальное — truthy. |
| **Логические `&&` / `\|\|`** | ✅ | Короткое замыкание, возвращают операнд (не boolean). |
| **Тернарный оператор** | ✅ | Корректная семантика. |
| **`typeof`** | ✅ | Возвращает корректные строки (`undefined`, `object`, `boolean`, `number`, `string`, `function`). |
| **Замыкания** | ✅ | Через цепочку лексических окружений (`Environment`). |
| **`this`** | ✅ | Привязка для методов классов и `obj.method()`. |
| **Microtask checkpoint** | ✅ | Микрозадачи выполняются до макрозадач, вновь добавленные — в рамках того же checkpoint. |
| **Event Loop порядок** | ✅ | Одна макрозадача → microtask checkpoint → рендер → idle. Соответствует WHATWG HTML. |
| **`async`/`await`** | ✅ | `await` ставит продолжение в микротаску; async-функция возвращает Promise. |

### Частичное соответствие ⚠️

| Аспект | Оценка | Комментарий |
|--------|--------|-------------|
| **`Promise`** | ⚠️ | Реализованы `resolve`, `then`, `catch`, `finally`, но модель упрощена: нет полноценного `reject`, нет обработки rejected-цепочки, `new Promise` не передаёт `reject` в executor. |
| **`==` приведение объектов** | ⚠️ | Объект→примитив реализован упрощённо (через строковое представление), без `valueOf`/`toString` протокола. |
| **`+` оператор** | ⚠️ | Корректно для чисел и строк, но не реализован полный `ToPrimitive` для объектов. |
| **Классы** | ⚠️ | `class`, конструкторы, методы работают; `extends` парсится, но наследование не реализовано. |
| **`setInterval`** | ⚠️ | Повторяется, но ограничен 3 итерациями (защита от зацикливания). |
| **`fetch`** | ⚠️ | Возвращает Promise, разрешающийся через макротаску, но с фиктивным ответом (status 200). |
| **`for`** | ⚠️ | Классический `for` работает; `for...of` / `for...in` не реализованы. |

### Не соответствует / не реализовано ❌

| Аспект | Комментарий |
|--------|-------------|
| **`reject` в Promise** | Не передаётся в executor; rejected-состояние не обрабатывается полноценно. |
| **`do...while`** | Не реализован (ключевое слово `do` есть в лексере, но парсер не обрабатывает). |
| **`for...of` / `for...in`** | Не реализованы (ключевые слова `of`/`in` есть в лексере). |
| **`Symbol`, `Map`, `Set`, `Proxy`, `Reflect`** | Не реализованы. |
| **Генераторы и `yield`** | Не реализованы. |
| **Модули ES (`import`/`export`)** | Не реализованы. |
| **`class extends` наследование** | Парсится, но не исполняется. |
| **`try` без `catch`** | Парсер требует `catch`; `try/finally` без `catch` не поддерживается. |
| **`switch` без `break` (fall-through)** | Реализован fall-through, но `break` внутри `switch` не обрабатывается корректно (обрабатывается как `break` цикла). |
| **`throw` объектов** | `throw` работает, но `catch` получает значение как строку (теряется тип). |
| **Строгий режим (`"use strict"`)** | Не реализован. |
| **`let`/`const` TDZ (temporal dead zone)** | Не реализован — переменные доступны до объявления. |
| **`var` hoisting** | Не реализован. |
| **`Number`/`String`/`Boolean` обёртки** | Не реализованы. |
| **`Array` методы** (`map`, `filter`, `reduce`, `push`, `pop` и т.д.) | Не реализованы. |
| **`Object` методы** (`keys`, `values`, `assign` и т.д.) | Не реализованы. |
| **`JSON`** | Не реализован. |
| **`Math`** | Не реализован. |
| **`Date`** | Не реализован. |
| **`Error` / исключения как объекты** | Ошибки представлены строками, не объектами `Error`. |
| **`arguments`** | Не реализован. |
| **`new` для произвольных функций** | Реализован только для `Promise` и классов. |
| **`instanceof`** | Не реализован. |
| **`in` / `delete`** | Не реализованы. |
| **Битовые операторы** (`&`, `\|`, `^`, `~`, `<<`, `>>`) | Токенизируются, но не исполняются. |
| **`**` (возведение в степень)** | Токенизируется, но не исполняется. |
| **`??` (nullish) / `?.` (optional chaining)** | Токенизируются, но не исполняются. |
| **`+=`, `-=`, `*=` и т.д.** | Токенизируются, но не исполняются (только `=`). |
| **`++`/`--` префиксные** | Реализованы только постфиксные (`i++`), префиксные (`++i`) не поддерживаются. |

### Итоговая оценка

Интерпретатор корректно реализует **ядро языка** (переменные, функции, замыкания,
управляющие конструкции, базовые операторы, `this`) и **механизм Event Loop**
(очереди, microtask/macrotask, рендер, rAF/rIC) в соответствии со спецификацией.
Однако он **не является полной реализацией ECMAScript**: отсутствуют многие
встроенные объекты (`Array`/`Object` методы, `JSON`, `Math`, `Date`, `Error`),
продвинутые конструкции (`for...of`, `do...while`, генераторы, модули, `Symbol`,
`Proxy`) и часть семантики Promise (полноценный `reject`, обработка rejected-цепочек).

Это осознанное ограничение: проект нацелен на **демонстрацию Event Loop**, а не на
полную совместимость с ECMAScript. Полная реализация — многолетний проект уровня V8.

---

## 📅 План реализации

См. [`docs/roadmap.md`](docs/roadmap.md).
