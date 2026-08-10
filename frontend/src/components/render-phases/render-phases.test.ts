/**
 * Тесты для компонента RenderPhases.
 * Используется Vitest + jsdom.
 */

import { describe, it, expect, beforeEach } from 'vitest';
import { RenderPhases } from './render-phases';

// Регистрируем компонент, если ещё не зарегистрирован.
if (!customElements.get('render-phases')) {
  customElements.define('render-phases', RenderPhases);
}

describe('RenderPhases', () => {
  let phases: RenderPhases;

  beforeEach(() => {
    phases = document.createElement('render-phases') as RenderPhases;
    document.body.appendChild(phases);
  });

  it('отображает все 10 фаз рендера', () => {
    const phaseElements = phases.shadowRoot?.querySelectorAll('.phase');
    expect(phaseElements?.length).toBe(10);
  });

  it('устанавливает активную фазу', () => {
    phases.setActivePhase('Layout');
    const active = phases.shadowRoot?.querySelector('.phase.active');
    expect(active?.getAttribute('data-phase')).toBe('Layout');
  });

  it('помечает предыдущую фазу как завершённую', () => {
    phases.setActivePhase('ParsingHtml');
    phases.setActivePhase('ParsingCss');
    const completed = phases.shadowRoot?.querySelector('.phase.completed');
    expect(completed?.getAttribute('data-phase')).toBe('ParsingHtml');
  });

  it('сбрасывает состояние фаз', () => {
    phases.setActivePhase('Layout');
    phases.reset();
    const active = phases.shadowRoot?.querySelector('.phase.active');
    const completed = phases.shadowRoot?.querySelector('.phase.completed');
    expect(active).toBeNull();
    expect(completed).toBeNull();
  });

  it('завершает рендер: сбрасывает активную фазу и помечает её завершённой', () => {
    phases.setActivePhase('Compose');
    phases.clearActivePhase();
    const active = phases.shadowRoot?.querySelector('.phase.active');
    const completed = phases.shadowRoot?.querySelector('.phase.completed');
    expect(active).toBeNull();
    expect(completed?.getAttribute('data-phase')).toBe('Compose');
  });
});