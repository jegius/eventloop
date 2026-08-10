/**
 * Темплейт компонента редактора кода.
 *
 * Чистая функция, возвращающая HTML-разметку на основе состояния
 * контроллера. Не содержит логики и побочных эффектов.
 */

/** Экранирует HTML-спецсимволы. */
function escapeHtml(text: string): string {
  return text
    .replace(/&/g, '&')
    .replace(/</g, '<')
    .replace(/>/g, '>')
    .replace(/"/g, '"');
}

/**
 * Возвращает HTML-разметку компонента редактора кода.
 *
 * @param code Текущий код.
 * @returns HTML-строка.
 */
export function codeEditorTemplate(code: string): string {
  return `
    <div class="editor">
      <div class="editor-header">
        <span class="title">JavaScript</span>
        <span class="badge">ECMAScript 2026</span>
      </div>
      <div class="editor-body">
        <div class="line-numbers" aria-hidden="true"></div>
        <div class="highlight" aria-hidden="true"></div>
        <textarea
          class="code-input"
          spellcheck="false"
          placeholder="Введите JavaScript-код..."
        >${escapeHtml(code)}</textarea>
      </div>
      <div class="editor-footer">
        <button class="btn btn-run" id="run-btn">▶ Выполнить</button>
        <button class="btn" id="clear-btn">Очистить</button>
      </div>
      <details class="limits" open>
        <summary>⚠️ Возможности интерпретатора</summary>
        <ul>
          <li>Синтаксис: <code>let/const/var</code>, <code>if/else</code>, <code>while</code>, <code>for</code>, <code>switch</code>, <code>break/continue</code>, <code>try/catch/finally</code>, <code>throw</code>, функции, стрелочные функции, классы, тернарный оператор, <code>typeof</code>, арифметика, сравнение, объекты, массивы.</li>
          <li>Асинхронность: <code>setTimeout</code>, <code>setInterval</code>, <code>setImmediate</code>, <code>Promise</code> (<code>new</code>, <code>.then</code>, <code>.catch</code>, <code>.finally</code>), <code>async/await</code>, <code>queueMicrotask</code>, <code>requestAnimationFrame</code>, <code>requestIdleCallback</code>, <code>fetch</code>.</li>
          <li>События: <code>document</code>/<code>window</code> <code>addEventListener</code> и <code>dispatchEvent</code>.</li>
          <li>Очереди: microtask (Promise, queueMicrotask), macrotask (setTimeout, setInterval, setImmediate, fetch, события), rAF, rIC, render.</li>
          <li>Рендер выполняется по упрощённой схеме (8 фаз, без реального HTML/CSS).</li>
        </ul>
      </details>
    </div>
  `;
}