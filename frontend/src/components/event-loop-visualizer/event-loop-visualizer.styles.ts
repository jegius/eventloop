/**
 * Стили главного компонента визуализатора Event Loop.
 *
 * Чистая функция, возвращающая CSS-строку для Shadow DOM компонента.
 */

/**
 * Возвращает CSS-стили главного компонента визуализатора.
 *
 * @returns CSS-строка.
 */
export function eventLoopVisualizerStyles(): string {
  return `
    :host {
      display: block;
      height: 100vh;
      width: 100%;
    }
    .visualizer {
      display: flex;
      flex-direction: column;
      height: 100%;
    }
    .toolbar {
      display: flex;
      align-items: center;
      justify-content: space-between;
      padding: 10px 16px;
      background: var(--color-surface);
      border-bottom: 1px solid var(--color-border);
    }
    .brand {
      display: flex;
      align-items: center;
      gap: 10px;
    }
    .logo {
      font-size: 20px;
    }
    .brand-name {
      font-weight: 700;
      font-size: 15px;
    }
    .badge {
      font-size: 11px;
      color: var(--color-accent);
      background: rgba(79, 140, 255, 0.15);
      padding: 2px 8px;
      border-radius: var(--radius-sm);
    }
    .controls {
      display: flex;
      align-items: center;
      gap: 12px;
    }
    .mode-toggle {
      display: flex;
      background: var(--color-surface-2);
      border-radius: var(--radius-sm);
      overflow: hidden;
    }
    .mode-btn {
      padding: 6px 12px;
      border: none;
      background: transparent;
      color: var(--color-text-muted);
      cursor: pointer;
      font-size: 12px;
    }
    .mode-btn.active {
      background: var(--color-accent);
      color: #fff;
    }
    .speed-control {
      display: flex;
      align-items: center;
      gap: 8px;
      font-size: 12px;
      color: var(--color-text-muted);
    }
    .speed-control input {
      width: 100px;
    }
    .btn {
      padding: 6px 14px;
      border: none;
      border-radius: var(--radius-sm);
      cursor: pointer;
      font-size: 12px;
      font-weight: 600;
      background: var(--color-surface-2);
      color: var(--color-text);
      transition: opacity 0.2s;
    }
    .btn:hover {
      opacity: 0.85;
    }
    .btn:disabled {
      opacity: 0.4;
      cursor: not-allowed;
    }
    .btn-play {
      background: var(--color-success);
      color: #000;
    }
    .main-layout {
      flex: 1;
      display: grid;
      grid-template-columns: 1fr 1.4fr 1fr;
      gap: 12px;
      padding: 12px;
      overflow: hidden;
    }
    .left-panel,
    .center-panel,
    .right-panel {
      display: flex;
      flex-direction: column;
      gap: 12px;
      min-height: 0;
    }
    .editor-container {
      flex: 1.6;
      min-height: 0;
    }
    .console-container {
      flex: 1;
      min-height: 0;
    }
    .center-panel {
      position: relative;
    }
    .execution-indicator-container {
      position: absolute;
      top: 0;
      left: 0;
      right: 0;
      bottom: 0;
      z-index: 10;
      pointer-events: none;
    }
    .call-stack-container {
      flex: 0 0 200px;
    }
    .queues-grid {
      flex: 1;
      display: grid;
      grid-template-columns: repeat(2, 1fr);
      gap: 12px;
      overflow-y: auto;
    }
    .queue-cell {
      min-height: 120px;
    }
    .render-phases-container {
      flex: 0 0 260px;
    }
    .event-log {
      flex: 1;
      display: flex;
      flex-direction: column;
      background: var(--color-surface);
      border: 1px solid var(--color-border);
      border-radius: var(--radius);
      overflow: hidden;
    }
    .log-header {
      display: flex;
      align-items: center;
      gap: 8px;
      padding: 8px 12px;
      background: var(--color-surface-2);
      border-bottom: 1px solid var(--color-border);
      font-weight: 600;
      font-size: 13px;
    }
    .log-header .log-count {
      margin-left: auto;
    }
    .btn-clear-log {
      padding: 3px 10px;
      border: none;
      border-radius: var(--radius-sm);
      cursor: pointer;
      font-size: 11px;
      font-weight: 600;
      background: var(--color-surface);
      color: var(--color-text-muted);
      transition: opacity 0.2s;
    }
    .btn-clear-log:hover {
      opacity: 0.85;
      color: var(--color-text);
    }
    .log-body {
      flex: 1;
      overflow-y: auto;
      padding: 8px;
      font-family: var(--font-mono);
      font-size: 11px;
    }
    .log-entry {
      padding: 2px 4px;
      color: var(--color-text-muted);
      border-left: 2px solid transparent;
    }
    .log-entry.active {
      color: var(--color-text);
      border-left-color: var(--color-accent);
      background: rgba(79, 140, 255, 0.08);
    }
  `;
}