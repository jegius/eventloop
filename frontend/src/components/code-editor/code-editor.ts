/**
 * Компонент редактора кода.
 *
 * Позволяет вставлять JS-код для выполнения интерпретатором.
 * Поддерживает подсветку синтаксиса через highlight.js.
 * Логика и состояние вынесены в контроллер, разметка — в темплейт,
 * стили — в отдельный файл.
 */

import { BaseComponent } from '../base-component';
import { CodeEditorController } from './code-editor.controller';
import { codeEditorTemplate } from './code-editor.template';
import { codeEditorStyles } from './code-editor.styles';
import hljs from 'highlight.js/lib/core';
import javascript from 'highlight.js/lib/languages/javascript';

// Регистрируем язык JavaScript
hljs.registerLanguage('javascript', javascript);

export class CodeEditor extends BaseComponent {
  /** Пример кода по умолчанию. */
  static readonly DEFAULT_CODE = `// Сложный пример: демонстрация всех аспектов Event Loop
console.log('1. Синхронный код');

// setTimeout — макротаска
setTimeout(() => {
  console.log('2. Macrotask: setTimeout');
}, 0);

// Promise — микротаска
Promise.resolve().then(() => {
  console.log('3. Microtask: Promise');
});

// requestAnimationFrame — перед рендером
requestAnimationFrame(() => {
  console.log('4. rAF: requestAnimationFrame');
});

// requestIdleCallback — в свободное время
requestIdleCallback(() => {
  console.log('5. rIC: requestIdleCallback');
});

// fetch — асинхронный запрос
fetch('/api/data').then(() => {
  console.log('6. fetch: ответ получен');
});

// setInterval — повторяющаяся макротаска
setInterval(() => {
  console.log('7. setInterval: тик');
}, 100);

// async/await — асинхронная функция
async function asyncTask() {
  console.log('8. async: начало');
  await Promise.resolve();
  console.log('9. async: после await');
}
asyncTask();

console.log('10. Конец синхронного кода');
`;

  /** Контроллер состояния кода. */
  private readonly controller = new CodeEditorController(CodeEditor.DEFAULT_CODE);

  constructor() {
    super();
    // При изменении кода НЕ перерисовываем весь компонент (update()),
    // т.к. это перезаписывает innerHTML и пересоздаёт textarea,
    // что приводит к потере фокуса при вводе. Вместо этого обновляем
    // только подсветку и номера строк, сохраняя DOM и фокус.
    this.controller.setOnChange(() => {
      this.syncHighlight();
    });
  }

  protected render(): void {
    this.update();
    this.bindEvents();
    this.syncHighlight();
  }

  protected template(): string {
    return codeEditorTemplate(this.controller.getCode());
  }

  protected styles(): string {
    return codeEditorStyles();
  }

  /** Синхронизирует подсветку с введённым кодом. */
  private syncHighlight(): void {
    const input = this.query<HTMLTextAreaElement>('.code-input');
    const highlight = this.query<HTMLElement>('.highlight');
    if (!input || !highlight) return;

    const code = input.value;
    const highlighted = hljs.highlight(code, { language: 'javascript' }).value;
    // Разбиваем подсвеченный код на строки, чтобы можно было подсвечивать
    // отдельную строку целиком (не только номер в колонке).
    const lines = highlighted.split('\n');
    highlight.innerHTML = lines
      .map((line, i) => `<div class="code-line" data-line="${i + 1}">${line || ' '}</div>`)
      .join('\n');

    this.updateLineNumbers(code);
  }

  /** Заполняет колонку с номерами строк. */
  private updateLineNumbers(code: string): void {
    const lineNumbers = this.query<HTMLElement>('.line-numbers');
    if (!lineNumbers) return;

    const count = code.split('\n').length;
    let html = '';
    for (let i = 1; i <= count; i++) {
      html += `<div class="ln" data-line="${i}">${i}</div>`;
    }
    lineNumbers.innerHTML = html;
  }

  /** Подсвечивает строку с указанным номером (1-based). */
  highlightLine(line: number): void {
    this.highlightLineRange(line, line);
  }

  /** Подсвечивает диапазон строк от `start` до `end` включительно (1-based). */
  highlightLineRange(start: number, end: number): void {
    const lineNumbers = this.query<HTMLElement>('.line-numbers');
    const highlight = this.query<HTMLElement>('.highlight');
    if (!lineNumbers) return;

    // Сбрасываем активные номера строк и строки кода.
    lineNumbers.querySelectorAll('.ln.active').forEach((el) => el.classList.remove('active'));
    highlight?.querySelectorAll('.code-line.active').forEach((el) => el.classList.remove('active'));

    // Подсвечиваем все строки в диапазоне.
    for (let line = start; line <= end; line++) {
      const ln = lineNumbers.querySelector(`.ln[data-line="${line}"]`);
      if (ln) {
        ln.classList.add('active');
      }
      const codeLine = highlight?.querySelector(`.code-line[data-line="${line}"]`);
      if (codeLine) {
        codeLine.classList.add('active');
      }
    }
  }

  /** Привязывает обработчики событий. */
  private bindEvents(): void {
    const input = this.query<HTMLTextAreaElement>('.code-input');
    const highlight = this.query<HTMLElement>('.highlight');
    const runBtn = this.query<HTMLButtonElement>('#run-btn');
    const clearBtn = this.query<HTMLButtonElement>('#clear-btn');

    input?.addEventListener('input', () => {
      // Обновляем состояние контроллера. setCode() триггерит notify(),
      // который вызывает syncHighlight() (см. конструктор), поэтому
      // отдельный вызов syncHighlight() здесь не требуется.
      this.controller.setCode(input.value);
    });

    // Синхронизируем прокрутку подсветки с прокруткой textarea.
    input?.addEventListener('scroll', () => {
      if (highlight) {
        highlight.scrollTop = input.scrollTop;
        highlight.scrollLeft = input.scrollLeft;
      }
    });

    runBtn?.addEventListener('click', () => {
      // Читаем значение напрямую из textarea, чтобы учесть все изменения
      const currentCode = input?.value || this.controller.getCode() || CodeEditor.DEFAULT_CODE;
      this.dispatchEvent(
        new CustomEvent('run-code', {
          detail: { code: currentCode },
          bubbles: true,
          composed: true,
        })
      );
    });

    clearBtn?.addEventListener('click', () => {
      if (input) {
        input.value = '';
      }
      // clear() обновляет состояние контроллера и триггерит notify(),
      // который вызывает syncHighlight() (см. конструктор).
      this.controller.clear();
    });
  }
}

// Регистрация компонента
if (!customElements.get('code-editor')) {
  customElements.define('code-editor', CodeEditor);
}