/**
 * Контроллер компонента фаз рендера.
 *
 * Содержит состояние (активная и завершённые фазы) и бизнес-логику
 * управления фазами. Не зависит от DOM — рендеринг выполняется через
 * колбэк `onChange`.
 */

import type { RenderPhase } from '../../types';

/** Определение фазы рендера. */
export interface PhaseDef {
  key: RenderPhase;
  label: string;
  icon: string;
}

/** Все фазы рендера. */
export const PHASES: PhaseDef[] = [
  { key: 'AnimationFrame', label: 'requestAnimationFrame', icon: '🎞️' },
  { key: 'ParsingHtml', label: 'Parsing HTML', icon: '📄' },
  { key: 'ParsingCss', label: 'Parsing CSS', icon: '🎨' },
  { key: 'Dom', label: 'DOM', icon: '🌳' },
  { key: 'Cssom', label: 'CSSOM', icon: '📐' },
  { key: 'RenderTree', label: 'Render Tree', icon: '🌲' },
  { key: 'Layout', label: 'Layout', icon: '📏' },
  { key: 'Paint', label: 'Paint', icon: '🖌️' },
  { key: 'Compose', label: 'Compose', icon: '🧩' },
];

/**
 * Контроллер фаз рендера.
 *
 * Реализует паттерн Controller: управляет состоянием и логикой фаз,
 * уведомляя представление (компонент) об изменениях через колбэк.
 */
export class RenderPhasesController {
  /** Активная фаза. */
  private activePhase: RenderPhase | null = null;

  /** Пройденные фазы. */
  private completedPhases: Set<RenderPhase> = new Set();

  /** Колбэк уведомления об изменении состояния. */
  private onChange: (() => void) | null = null;

  /** Устанавливает колбэк уведомления об изменении. */
  setOnChange(callback: () => void): void {
    this.onChange = callback;
  }

  /** Возвращает активную фазу. */
  getActivePhase(): RenderPhase | null {
    return this.activePhase;
  }

  /** Возвращает завершённые фазы. */
  getCompletedPhases(): Set<RenderPhase> {
    return this.completedPhases;
  }

  /** Устанавливает активную фазу. */
  setActivePhase(phase: RenderPhase): void {
    if (this.activePhase) {
      this.completedPhases.add(this.activePhase);
    }
    this.activePhase = phase;
    this.notify();
  }

  /** Сбрасывает состояние фаз. */
  reset(): void {
    this.activePhase = null;
    this.completedPhases.clear();
    this.notify();
  }

  /** Уведомляет представление об изменении состояния. */
  private notify(): void {
    this.onChange?.();
  }
}