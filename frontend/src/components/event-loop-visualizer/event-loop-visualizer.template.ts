/**
 * Темплейт главного компонента визуализатора Event Loop.
 *
 * Чистая функция, возвращающая HTML-разметку на основе состояния
 * контроллера. Не содержит логики и побочных эффектов.
 */

import type { PlaybackMode } from '../../types';

/**
 * Возвращает HTML-разметку главного компонента визуализатора.
 *
 * @param mode             Режим работы (realtime / step).
 * @param speed            Скорость выполнения (мс на шаг).
 * @param playButtonLabel  Подпись кнопки воспроизведения.
 * @returns HTML-строка.
 */
export function eventLoopVisualizerTemplate(
  mode: PlaybackMode,
  speed: number,
  playButtonLabel: string
): string {
  return `
    <div class="visualizer">
      <header class="toolbar">
        <div class="brand">
          <span class="logo">⚡</span>
          <span class="brand-name">Event Loop Visualizer</span>
          <span class="badge">ECMAScript 2026</span>
        </div>
        <div class="controls">
          <div class="mode-toggle">
            <button class="mode-btn ${mode === 'realtime' ? 'active' : ''}" data-mode="realtime">Реальное время</button>
            <button class="mode-btn ${mode === 'step' ? 'active' : ''}" data-mode="step">Пошагово</button>
          </div>
          <div class="speed-control">
            <label>Скорость</label>
            <input type="range" min="100" max="2000" step="100" value="${speed}" id="speed-slider" />
            <span class="speed-value">${speed}ms</span>
          </div>
          <button class="btn btn-play" id="play-btn">${playButtonLabel}</button>
          <button class="btn" id="step-btn" ${mode === 'step' ? '' : 'disabled'}>⏭ Шаг</button>
          <button class="btn" id="reset-btn">↺ Сброс</button>
        </div>
      </header>

      <div class="main-layout">
        <div class="left-panel">
          <div class="editor-container">
            <code-editor></code-editor>
          </div>
          <div class="console-container">
            <event-loop-console></event-loop-console>
          </div>
        </div>

        <div class="center-panel">
          <div class="execution-indicator-container">
            <execution-indicator></execution-indicator>
          </div>
          <div class="call-stack-container">
            <call-stack></call-stack>
          </div>
          <div class="queues-grid">
            <div class="queue-cell">
              <event-loop-queue id="microtask-queue"></event-loop-queue>
            </div>
            <div class="queue-cell">
              <event-loop-queue id="macrotask-queue"></event-loop-queue>
            </div>
            <div class="queue-cell">
              <event-loop-queue id="render-queue"></event-loop-queue>
            </div>
          </div>
        </div>

        <div class="right-panel">
          <div class="render-phases-container">
            <render-phases></render-phases>
          </div>
          <div class="event-log">
            <div class="log-header">
              <span>Event Log</span>
              <span class="log-count" id="log-count">0</span>
            </div>
            <div class="log-body" id="log-body"></div>
          </div>
        </div>
      </div>
    </div>
  `;
}