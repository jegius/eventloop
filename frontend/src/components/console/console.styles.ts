/**
 * Стили компонента консоли вывода.
 *
 * Чистая функция, возвращающая CSS-строку для Shadow DOM компонента.
 */

/**
 * Возвращает CSS-стили компонента консоли.
 *
 * @returns CSS-строка.
 */
export function consoleStyles(): string {
  return `
    :host {
      display: block;
      height: 100%;
    }
    .console {
      display: flex;
      flex-direction: column;
      height: 100%;
      background: var(--color-surface);
      border: 1px solid var(--color-border);
      border-radius: var(--radius);
      overflow: hidden;
    }
    .console-header {
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
    .clear-btn {
      background: transparent;
      border: 1px solid var(--color-border);
      color: var(--color-text-muted);
      padding: 4px 10px;
      border-radius: var(--radius-sm);
      cursor: pointer;
      font-size: 12px;
    }
    .clear-btn:hover {
      color: var(--color-text);
      border-color: var(--color-accent);
    }
    .console-body {
      flex: 1;
      overflow-y: auto;
      padding: 8px 12px;
      font-family: var(--font-mono);
      font-size: 13px;
    }
    .entry {
      display: flex;
      gap: 8px;
      padding: 2px 0;
      line-height: 1.5;
    }
    .entry .prefix {
      color: var(--color-text-muted);
    }
    .entry.error .text {
      color: var(--color-danger);
    }
    .entry.info .text {
      color: var(--color-accent);
    }
    .entry.warning .text {
      color: var(--color-warning);
    }
    .empty {
      color: var(--color-text-muted);
      font-style: italic;
    }
  `;
}