/**
 * Стили компонента Call Stack.
 *
 * Чистая функция, возвращающая CSS-строку для Shadow DOM компонента.
 */

/**
 * Возвращает CSS-стили компонента Call Stack.
 *
 * @returns CSS-строка.
 */
export function callStackStyles(): string {
  return `
    :host {
      display: block;
      height: 100%;
    }
    .call-stack {
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
      border-bottom: 1px solid var(--color-border);
    }
    .title {
      font-weight: 600;
      font-size: 13px;
    }
    .count {
      background: var(--color-accent);
      color: #fff;
      font-size: 12px;
      font-weight: 600;
      padding: 2px 8px;
      border-radius: var(--radius-sm);
    }
    .stack-body {
      flex: 1;
      overflow-y: auto;
      padding: 8px;
      display: flex;
      flex-direction: column-reverse;
    }
    .stack-item {
      display: flex;
      align-items: center;
      gap: 8px;
      padding: 6px 10px;
      margin-bottom: 4px;
      background: var(--color-surface-2);
      border: 1px solid var(--color-border);
      border-radius: var(--radius-sm);
      font-family: var(--font-mono);
      font-size: 12px;
      animation: slideIn 0.2s ease;
    }
    .stack-item.active {
      border-color: var(--color-accent);
      background: rgba(79, 140, 255, 0.1);
    }
    .stack-body.distributing {
      animation: distribute 0.3s ease;
    }
    @keyframes distribute {
      0% { box-shadow: inset 0 0 0 0 rgba(79, 140, 255, 0); }
      50% { box-shadow: inset 0 0 0 2px rgba(79, 140, 255, 0.6); }
      100% { box-shadow: inset 0 0 0 0 rgba(79, 140, 255, 0); }
    }
    .stack-item .index {
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
      from { opacity: 0; transform: translateY(-4px); }
      to { opacity: 1; transform: translateY(0); }
    }
  `;
}