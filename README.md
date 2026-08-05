# JS Event Loop Visualizer

Интерактивный визуализатор Event Loop JavaScript согласно спецификации ECMAScript 2026.

## 🎯 Описание проекта

Проект представляет собой полноценный инструмент для визуализации работы Event Loop в JavaScript, включая:
- Call Stack (стек вызовов)
- Microtask Queue (очередь микрозадач)
- Macrotask Queue (очередь макрозадач)
- Render Queue (очередь рендеринга)
- Request Animation Frame
- Request Idle Callback
- Все фазы рендеринга браузера

## 🏗️ Архитектура

### Backend (Rust)
Высокопроизводительный сервер на Rust, реализующий интерпретатор JavaScript с поддержкой Event Loop.

**Компоненты:**
- `backend/src/models.rs` - Модели данных для всех очередей и состояний
- `backend/src/event_loop.rs` - Движок Event Loop согласно ECMAScript 2026
- `backend/src/main.rs` - WebSocket сервер для коммуникации с фронтендом

**Поддерживаемые API:**
- `setTimeout` / `setInterval`
- `Promise` (resolve/reject/then/catch/finally)
- `queueMicrotask`
- `requestAnimationFrame`
- `requestIdleCallback`
- `console.*` методы

### Frontend (TypeScript + Web Components)
Современный интерфейс на базе Web Components для визуализации работы Event Loop.

**Компоненты:**
- `el-call-stack` - Визуализация стека вызовов
- `el-task-queue` - Очереди задач (микро/макро/render/raf/ric)
- `el-render-phases` - Фазы рендеринга
- `el-console-output` - Консоль в стиле V8
- `el-app` - Главное приложение

**Технологии:**
- TypeScript 5.3+
- Lit 3.0 (Web Components)
- Vite 5.0 (сборка)
- WebSocket для real-time коммуникации

## 📋 План реализации

### Фаза 1: Базовая инфраструктура ✅
- [x] Инициализация проекта
- [x] Настройка backend (Rust + Cargo)
- [x] Настройка frontend (TypeScript + Vite)
- [x] WebSocket коммуникация
- [x] Базовые модели данных
- [x] Web Components для визуализации

### Фаза 2: Event Loop Engine
- [ ] Полный парсер JavaScript (на базе swc или similar)
- [ ] Реализация всех фаз Event Loop по спецификации
- [ ] Поддержка checkpoint и performance tasks
- [ ] User interaction tasks
- [ ] GC симуляция

### Фаза 3: Browser API
- [ ] Полная поддержка таймеров
- [ ] Promise с правильной очередью микрозадач
- [ ] DOM Events симуляция
- [ ] Network events (fetch/XMLHttpRequest)
- [ ] Performance API

### Фаза 4: Визуализация
- [ ] Детальная визуализация каждой фазы
- [ ] Анимации перехода между состояниями
- [ ] Интерактивные breakpoints
- [ ] История выполнения с rewind
- [ ] Экспорт/импорт состояний

### Фаза 5: Продвинутые функции
- [ ] Поддержка async/await
- [ ] Generators и Iterators
- [ ] Web Workers симуляция
- [ ] Service Workers
- [ ] Offscreen Canvas

## 🚀 Запуск

### Backend
```bash
cd backend
cargo run
```

Сервер запустится на `ws://localhost:8080`

### Frontend
```bash
cd frontend
npm install
npm run dev
```

Приложение откроется на `http://localhost:3000`

## 📖 Использование

1. Откройте приложение в браузере
2. Введите JavaScript код в редактор
3. Нажмите "Run" для выполнения
4. Используйте кнопки управления для пошагового выполнения:
   - ▶️ Play - непрерывное выполнение
   - ⏸️ Pause - пауза
   - ⏹️ Stop - остановка
   - ⏭️ Step Over - шаг через функцию
   - ⤵️ Step Into - вход в функцию
   - ⤴️ Step Out - выход из функции
   - ⏪ Rewind - возврат назад

5. Наблюдайте за изменением очередей и стека вызовов
6. Изучайте фазы рендеринга
7. Смотрите вывод в консоли

## 🎨 Дизайн

Современный темный интерфейс с:
- Градиентными акцентами
- Плавными анимациями
- Адаптивной версткой
- Интуитивной навигацией
- Цветовой схемой, соответствующей современным UX требованиям

## 📚 Необходимые навыки

Для разработки проекта требуются:

### Backend
- Rust (tokio, serde, WebSocket)
- Понимание ECMAScript спецификации
- Алгоритмы Event Loop
- Парсинг и AST

### Frontend
- TypeScript
- Web Components (Lit)
- CSS Grid/Flexbox
- Анимации CSS
- WebSocket API

### Общие
- Понимание архитектуры браузеров
- Спецификация ECMAScript 2026
- Event Loop алгоритмы
- Browser rendering pipeline

## 📄 Лицензия

MIT

## 🔗 Ресурсы

- [ECMAScript 2026 Specification](https://tc39.es/ecma262/)
- [HTML Living Standard - Event Loop](https://html.spec.whatwg.org/multipage/webappapis.html#event-loop-processing-model)
- [Jake Archibald - In the loop](https://www.youtube.com/watch?v=cCOL7MC4Pl0)