/**
 * Стили компонента очереди Event Loop.
 *
 * Чистая функция, возвращающая CSS-строку для Shadow DOM компонента.
 */

/**
 * Возвращает CSS-стили компонента очереди.
 *
 * @returns CSS-строка.
 */
export function queueStyles(): string {
  return `
    :host {
      display: block;
      height: 100%;
    }
    .queue {
      display: flex;
      flex-direction: column;
      height: 100%;
      background: var(--color-surface);
      border: 1px solid var(--color-border);
      border-radius: var(--radius);
      overflow: hidden;
    }
    .header {
      display: flex;
      align-items: center;
      justify-content: space-between;
      padding: 8px 12px;
      background: var(--color-surface-2);
      border-bottom: 2px solid;
    }
    .title {
      font-weight: 600;
      font-size: 13px;
    }
    .count {
      color: #fff;
      font-size: 12px;
      font-weight: 600;
      padding: 2px 8px;
      border-radius: var(--radius-sm);
    }
    .desc {
      padding: 6px 12px;
      font-size: 11px;
      color: var(--color-text-muted);
      border-bottom: 1px solid var(--color-border);
    }
    .queue-body {
      flex: 1;
      overflow-y: auto;
      padding: 8px;
    }
    .queue-item {
      display: flex;
      align-items: center;
      gap: 8px;
      padding: 6px 10px;
      margin-bottom: 4px;
      background: var(--color-surface-2);
      border: 1px solid var(--color-border);
      border-left: 3px solid;
      border-radius: var(--radius-sm);
      font-family: var(--font-mono);
      font-size: 12px;
      animation: slideIn 0.2s ease;
    }
    .queue-item .id {
      color: var(--color-text-muted);
      font-size: 10px;
    }
    .empty {
      color: var(--color-text-muted);
      font-style: italic;
      text-align: center;
      padding: 20px;
    }
    @keyframes slideIn {
      from { opacity: 0; transform: translateX(-4px); }
      to { opacity: 1; transform: translateX(0); }
    }
    @keyframes slideOut {
      from { opacity: 1; transform: translateX(0); }
      to { opacity: 0; transform: translateX(4px); }
    }
    .queue-item.leaving {
      animation: slideOut 0.2s ease forwards;
    }
  `;
}