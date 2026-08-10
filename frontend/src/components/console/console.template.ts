/**
 * Темплейт компонента консоли вывода.
 *
 * Чистая функция, возвращающая HTML-разметку на основе состояния
 * контроллера. Не содержит логики и побочных эффектов.
 */

import type { ConsoleEntry } from './console.controller';

/** Экранирует HTML-спецсимволы. */
function escapeHtml(text: string): string {
  return text.replace(/&/g, '&').replace(/</g, '<').replace(/>/g, '>');
}

/**
 * Возвращает HTML-разметку компонента консоли.
 *
 * @param entries Записи консоли.
 * @returns HTML-строка.
 */
export function consoleTemplate(entries: ConsoleEntry[]): string {
  const entriesHtml = entries
    .map(
      (entry) => `
        <div class="entry ${entry.type}">
          <span class="prefix">${entry.type === 'error' ? '✖' : entry.type === 'warning' ? '⚠' : '›'}</span>
          <span class="text">${escapeHtml(entry.text)}</span>
        </div>
      `
    )
    .join('');

  return `
    <div class="console">
      <div class="console-header">
        <span class="title">Консоль</span>
        <button class="clear-btn" id="clear-btn">Очистить</button>
      </div>
      <div class="console-body" id="console-body">
        ${entriesHtml || '<div class="empty">Вывод появится здесь...</div>'}
      </div>
    </div>
  `;
}