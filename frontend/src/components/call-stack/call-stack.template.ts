/**
 * Темплейт компонента Call Stack.
 *
 * Чистая функция, возвращающая HTML-разметку на основе состояния
 * контроллера. Не содержит логики и побочных эффектов.
 */

import type { StackItem } from '../../types';

/** Экранирует HTML-спецсимволы. */
function escapeHtml(text: string): string {
  return text.replace(/&/g, '&').replace(/</g, '<').replace(/>/g, '>');
}

/**
 * Возвращает HTML-разметку компонента Call Stack.
 *
 * @param items Элементы стека.
 * @returns HTML-строка.
 */
export function callStackTemplate(items: StackItem[]): string {
  const itemsHtml = items
    .map(
      (item, index) => `
        <div class="stack-item ${index === items.length - 1 ? 'active' : ''}">
          <span class="index">${index}</span>
          <span class="label">${escapeHtml(item.label)}</span>
        </div>
      `
    )
    .join('');

  return `
    <div class="call-stack">
      <div class="header">
        <span class="title">Call Stack</span>
        <span class="count">${items.length}</span>
      </div>
      <div class="stack-body">
        ${itemsHtml || '<div class="empty">Стек пуст</div>'}
      </div>
    </div>
  `;
}