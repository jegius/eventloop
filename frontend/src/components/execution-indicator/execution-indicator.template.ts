/**
 * Темплейт компонента индикатора потока исполнения.
 *
 * Чистая функция, возвращающая HTML-разметку на основе текущей позиции
 * потока. Не содержит логики и побочных эффектов.
 */

import type { ExecutionPosition } from './execution-indicator.controller';

/**
 * Возвращает HTML-разметку индикатора потока исполнения.
 *
 * @param position Текущая позиция потока.
 * @param label    Подпись текущей позиции.
 * @returns HTML-строка.
 */
export function executionIndicatorTemplate(
  position: ExecutionPosition,
  label: string
): string {
  return `
    <div class="execution-indicator" data-position="${position}">
      <div class="indicator-dot"></div>
      <div class="indicator-pulse"></div>
      <div class="indicator-label">
        <span class="indicator-title">Поток исполнения</span>
        <span class="indicator-position">${label}</span>
      </div>
    </div>
  `;
}