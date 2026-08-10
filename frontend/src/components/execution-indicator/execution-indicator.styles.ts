/**
 * Стили компонента индикатора потока исполнения.
 *
 * Чистая функция, возвращающая CSS-строку для Shadow DOM компонента.
 * Анимированный значок, который перемещается по экрану между очередями
 * Event Loop, показывая текущее положение потока исполнения.
 */

/**
 * Возвращает CSS-стили индикатора потока исполнения.
 *
 * @returns CSS-строка.
 */
export function executionIndicatorStyles(): string {
  return `
    :host {
      display: block;
      position: relative;
      width: 100%;
      height: 100%;
      pointer-events: none;
    }
    .execution-indicator {
      position: absolute;
      top: 0;
      left: 0;
      display: flex;
      align-items: center;
      gap: 8px;
      padding: 6px 12px;
      background: var(--color-surface);
      border: 1px solid var(--color-border);
      border-radius: var(--radius);
      box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
      transform: translate(0, 0);
      transition: transform 0.3s cubic-bezier(0.4, 0, 0.2, 1);
      will-change: transform;
      z-index: 10;
    }
    .indicator-dot {
      width: 12px;
      height: 12px;
      border-radius: 50%;
      background: var(--color-accent);
      box-shadow: 0 0 8px var(--color-accent);
      transition: background 0.3s ease, box-shadow 0.3s ease;
      flex-shrink: 0;
    }
    .indicator-pulse {
      position: absolute;
      left: 12px;
      width: 12px;
      height: 12px;
      border-radius: 50%;
      border: 2px solid var(--color-accent);
      opacity: 0;
      animation: pulse 1.6s ease-out infinite;
      pointer-events: none;
    }
    .indicator-label {
      display: flex;
      flex-direction: column;
      gap: 1px;
      white-space: nowrap;
    }
    .indicator-title {
      font-size: 10px;
      color: var(--color-text-muted);
      text-transform: uppercase;
      letter-spacing: 0.5px;
    }
    .indicator-position {
      font-size: 12px;
      font-weight: 600;
      color: var(--color-text);
    }
    /* Позиция: поток не активен — серый, без пульсации. */
    .execution-indicator[data-position="idle"] .indicator-dot {
      background: var(--color-text-muted);
      box-shadow: none;
    }
    .execution-indicator[data-position="idle"] .indicator-pulse {
      display: none;
    }
    /* Позиция: Call Stack — синий. */
    .execution-indicator[data-position="callstack"] .indicator-dot {
      background: var(--color-accent);
      box-shadow: 0 0 8px var(--color-accent);
    }
    .execution-indicator[data-position="callstack"] .indicator-pulse {
      border-color: var(--color-accent);
    }
    /* Позиция: Microtask — зелёный. */
    .execution-indicator[data-position="microtask"] .indicator-dot {
      background: var(--color-microtask);
      box-shadow: 0 0 8px var(--color-microtask);
    }
    .execution-indicator[data-position="microtask"] .indicator-pulse {
      border-color: var(--color-microtask);
    }
    /* Позиция: Macrotask — оранжевый. */
    .execution-indicator[data-position="macrotask"] .indicator-dot {
      background: var(--color-macrotask);
      box-shadow: 0 0 8px var(--color-macrotask);
    }
    .execution-indicator[data-position="macrotask"] .indicator-pulse {
      border-color: var(--color-macrotask);
    }
    /* Позиция: Render — красный. */
    .execution-indicator[data-position="render"] .indicator-dot {
      background: var(--color-render);
      box-shadow: 0 0 8px var(--color-render);
    }
    .execution-indicator[data-position="render"] .indicator-pulse {
      border-color: var(--color-render);
    }
    /* Позиция: rIC — фиолетовый. */
    .execution-indicator[data-position="ric"] .indicator-dot {
      background: var(--color-ric);
      box-shadow: 0 0 8px var(--color-ric);
    }
    .execution-indicator[data-position="ric"] .indicator-pulse {
      border-color: var(--color-ric);
    }
    @keyframes pulse {
      0% {
        transform: scale(0.6);
        opacity: 0.8;
      }
      100% {
        transform: scale(1.4);
        opacity: 0;
      }
    }
  `;
}