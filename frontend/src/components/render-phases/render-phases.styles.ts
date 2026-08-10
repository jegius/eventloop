/**
 * Стили компонента фаз рендера.
 *
 * Чистая функция, возвращающая CSS-строку для Shadow DOM компонента.
 */

/**
 * Возвращает CSS-стили компонента фаз рендера.
 *
 * @returns CSS-строка.
 */
export function renderPhasesStyles(): string {
  return `
    :host {
      display: block;
      height: 100%;
    }
    .render-phases {
      display: flex;
      flex-direction: column;
      height: 100%;
      background: var(--color-surface);
      border: 1px solid var(--color-border);
      border-radius: var(--radius);
      overflow: hidden;
    }
    .header {
      padding: 8px 12px;
      background: var(--color-surface-2);
      border-bottom: 1px solid var(--color-border);
    }
    .title {
      font-weight: 600;
      font-size: 13px;
    }
    .phases-grid {
      flex: 1;
      display: grid;
      grid-template-columns: repeat(2, 1fr);
      gap: 8px;
      padding: 10px;
      overflow-y: auto;
    }
    .phase {
      display: flex;
      align-items: center;
      gap: 8px;
      padding: 10px;
      background: var(--color-surface-2);
      border: 1px solid var(--color-border);
      border-radius: var(--radius-sm);
      font-size: 12px;
      transition: all 0.2s;
      position: relative;
    }
    .phase .icon {
      font-size: 16px;
    }
    .phase.active {
      border-color: var(--color-render);
      background: rgba(255, 92, 92, 0.1);
      box-shadow: 0 0 8px rgba(255, 92, 92, 0.3);
    }
    .phase.completed {
      border-color: var(--color-success);
      opacity: 0.7;
    }
    .phase .indicator {
      position: absolute;
      top: 4px;
      right: 6px;
      color: var(--color-render);
      font-size: 10px;
      animation: pulse 1s infinite;
    }
    @keyframes pulse {
      0%, 100% { opacity: 1; }
      50% { opacity: 0.3; }
    }
  `;
}