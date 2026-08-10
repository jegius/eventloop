/**
 * Компонент визуализации фаз рендера.
 *
 * Отображает 8 фаз: Parsing HTML, Parsing CSS, DOM, CSSOM,
 * Render Tree, Layout, Paint, Compose.
 * Логика и состояние вынесены в контроллер, разметка — в темплейт,
 * стили — в отдельный файл.
 */

import { BaseComponent } from '../base-component';
import { RenderPhasesController } from './render-phases.controller';
import { renderPhasesTemplate } from './render-phases.template';
import { renderPhasesStyles } from './render-phases.styles';
import type { RenderPhase } from '../../types';

export class RenderPhases extends BaseComponent {
  /** Контроллер состояния фаз. */
  private readonly controller = new RenderPhasesController();

  constructor() {
    super();
    // При изменении состояния перерисовываем компонент.
    this.controller.setOnChange(() => this.update());
  }

  protected render(): void {
    this.update();
  }

  protected template(): string {
    return renderPhasesTemplate(
      this.controller.getActivePhase(),
      this.controller.getCompletedPhases()
    );
  }

  protected styles(): string {
    return renderPhasesStyles();
  }

  /** Устанавливает активную фазу. */
  public setActivePhase(phase: RenderPhase): void {
    this.controller.setActivePhase(phase);
  }

  /** Завершает рендер: сбрасывает активную фазу. */
  public clearActivePhase(): void {
    this.controller.clearActivePhase();
  }

  /** Сбрасывает состояние фаз. */
  public reset(): void {
    this.controller.reset();
  }
}

// Регистрация компонента
if (!customElements.get('render-phases')) {
  customElements.define('render-phases', RenderPhases);
}