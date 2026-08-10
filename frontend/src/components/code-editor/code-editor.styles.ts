/**
 * Стили компонента редактора кода.
 *
 * Чистая функция, возвращающая CSS-строку для Shadow DOM компонента.
 */

/**
 * Возвращает CSS-стили компонента редактора кода.
 *
 * @returns CSS-строка.
 */
export function codeEditorStyles(): string {
  return `
    :host {
      display: block;
      height: 100%;
    }
    .editor {
      display: flex;
      flex-direction: column;
      height: 100%;
      background: var(--color-surface);
      border: 1px solid var(--color-border);
      border-radius: var(--radius);
      overflow: hidden;
    }
    .editor-header {
      display: flex;
      align-items: center;
      justify-content: space-between;
      padding: 8px 12px;
      background: var(--color-surface-2);
      border-bottom: 1px solid var(--color-border);
    }
    .title {
      font-weight: 600;
      font-size: 13px;
    }
    .badge {
      font-size: 11px;
      color: var(--color-accent);
      background: rgba(79, 140, 255, 0.15);
      padding: 2px 8px;
      border-radius: var(--radius-sm);
    }
    .editor-body {
      position: relative;
      flex: 1;
      overflow: hidden;
    }
    .line-numbers {
      position: absolute;
      top: 0;
      left: 0;
      width: 44px;
      height: 100%;
      padding: 12px 0;
      overflow: hidden;
      background: var(--color-surface-2);
      border-right: 1px solid var(--color-border);
      font-family: var(--font-mono);
      font-size: 13px;
      line-height: 1.6;
      text-align: right;
      color: var(--color-text-muted);
      user-select: none;
      z-index: 3;
    }
    .line-numbers .ln {
      padding-right: 8px;
      white-space: pre;
    }
    .line-numbers .ln.active {
      color: var(--color-accent);
      font-weight: 700;
      background: rgba(79, 140, 255, 0.15);
    }
    .highlight,
    .code-input {
      position: absolute;
      top: 0;
      left: 0;
      width: 100%;
      height: 100%;
      margin: 0;
      padding: 12px;
      border: none;
      font-family: var(--font-mono);
      font-size: 13px;
      line-height: 1.6;
      tab-size: 2;
      overflow: auto;
      box-sizing: border-box;
    }
    .highlight {
      pointer-events: none;
      color: var(--color-text);
      z-index: 1;
    }
    .highlight,
    .code-input {
      padding-left: 56px;
    }
    .highlight .hljs-keyword { color: #c678dd; }
    .highlight .hljs-string { color: #98c379; }
    .highlight .hljs-number { color: #d19a66; }
    .highlight .hljs-comment { color: #5c6370; font-style: italic; }
    .highlight .hljs-function { color: #61afef; }
    .highlight .hljs-title { color: #61afef; }
    .highlight .hljs-params { color: #e6e6e6; }
    .highlight .hljs-built_in { color: #e06c75; }
    .highlight .hljs-literal { color: #56b6c2; }
    .code-line {
      display: block;
      min-height: 1.6em;
      white-space: pre-wrap;
      word-wrap: break-word;
      width: 100%;
    }
    .code-line.active {
      background: rgba(79, 140, 255, 0.12);
    }
    .code-input {
      z-index: 2;
      background: transparent;
      color: transparent;
      outline: none;
      resize: none;
      caret-color: var(--color-text);
      -webkit-text-fill-color: transparent;
      white-space: pre-wrap;
      word-wrap: break-word;
    }
    .code-input::selection {
      background: rgba(79, 140, 255, 0.3);
      -webkit-text-fill-color: transparent;
    }
    .code-input::placeholder {
      color: var(--color-text-muted);
    }
    .editor-footer {
      display: flex;
      gap: 8px;
      padding: 10px 12px;
      border-top: 1px solid var(--color-border);
    }
    .limits {
      border-top: 1px solid var(--color-border);
      background: var(--color-surface-2);
      font-size: 11px;
      color: var(--color-text-muted);
    }
    .limits summary {
      cursor: pointer;
      padding: 8px 12px;
      font-weight: 600;
      color: var(--color-warning);
      user-select: none;
    }
    .limits summary:hover {
      opacity: 0.85;
    }
    .limits ul {
      margin: 0;
      padding: 0 12px 10px 28px;
      display: flex;
      flex-direction: column;
      gap: 4px;
    }
    .limits li {
      line-height: 1.4;
    }
    .limits code {
      font-family: var(--font-mono);
      color: var(--color-accent);
      background: rgba(79, 140, 255, 0.1);
      padding: 0 4px;
      border-radius: 3px;
    }
    .btn {
      padding: 8px 16px;
      border: none;
      border-radius: var(--radius-sm);
      cursor: pointer;
      font-size: 13px;
      font-weight: 600;
      transition: opacity 0.2s;
    }
    .btn:hover {
      opacity: 0.85;
    }
    .btn-run {
      background: var(--color-accent);
      color: #fff;
    }
    #btn-clear {
      background: var(--color-surface-2);
      color: var(--color-text);
    }
  `;
}