/**
 * Темплейт компонента очереди Event Loop.
 *
 * Чистая функция, возвращающая HTML-разметку на основе состояния
 * контроллера. Не содержит логики и побочных эффектов.
 */

import type { QueueItem } from '../../types';
import type { QueueConfig } from './queue.controller';

/** Экранирует HTML-спецсимволы. */
function escapeHtml(text: string): string {
  return text.replace(/&/g, '&').replace(/</g, '<').replace(/>/g, '>');
}

/**
 * Возвращает HTML-разметку компонента очереди.
 *
 * @param config Конфигурация очереди.
 * @param items  Элементы очереди.
 * @returns HTML-строка.
 */
export function queueTemplate(config: QueueConfig, items: QueueItem[]): string {
  const itemsHtml = items
    .map(
      (item) => `
        <div class="queue-item" style="border-left-color: ${config.color}">
          <span class="id">#${item.id}</span>
          <span class="label">${escapeHtml(item.label)}</span>
        </div>
      `
    )
    .join('');

  return `
    <div class="queue">
      <div class="header" style="border-bottom-color: ${config.color}">
        <span class="title">${config.title}</span>
        <span class="count" style="background: ${config.color}">${items.length}</span>
      </div>
      ${config.description ? `<div class="desc">${config.description}</div>` : ''}
      <div class="queue-body">
        ${itemsHtml || '<div class="empty">Очередь пуста</div>'}
      </div>
    </div>
  `;
}