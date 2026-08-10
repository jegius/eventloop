/**
 * Темплейт компонента фаз рендера.
 *
 * Чистая функция, возвращающая HTML-разметку на основе состояния
 * контроллера. Не содержит логики и побочных эффектов.
 */

import type { RenderPhase } from '../../types';
import { PHASES } from './render-phases.controller';

/**
 * Возвращает HTML-разметку компонента фаз рендера.
 *
 * @param activePhase     Активная фаза.
 * @param completedPhases Завершённые фазы.
 * @returns HTML-строка.
 */
export function renderPhasesTemplate(
  activePhase: RenderPhase | null,
  completedPhases: Set<RenderPhase>
): string {
  const phasesHtml = PHASES.map((phase) => {
    const isActive = activePhase === phase.key;
    const isCompleted = completedPhases.has(phase.key);
    const stateClass = isActive ? 'active' : isCompleted ? 'completed' : '';

    return `
      <div class="phase ${stateClass}" data-phase="${phase.key}">
        <span class="icon">${phase.icon}</span>
        <span class="label">${phase.label}</span>
        ${isActive ? '<span class="indicator">●</span>' : ''}
      </div>
    `;
  }).join('');

  return `
    <div class="render-phases">
      <div class="header">
        <span class="title">Render Phases</span>
      </div>
      <div class="phases-grid">
        ${phasesHtml}
      </div>
    </div>
  `;
}