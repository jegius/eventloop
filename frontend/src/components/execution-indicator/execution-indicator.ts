/**
 * Компонент индикатора потока исполнения.
 *
 * Показывает анимированный значок, который перемещается по экрану между
 * очередями Event Loop и Call Stack, обозначая текущее положение
 * интерпретатора.
 *
 * Логика и состояние вынесены в контроллер, разметка — в темплейт,
 * стили — в отдельный файл.
 */

import { BaseComponent } from '../base-component';
import {
  ExecutionIndicatorController,
  type ExecutionPosition,
  type PositionCoords,
} from './execution-indicator.controller';
import { executionIndicatorTemplate } from './execution-indicator.template';
import { executionIndicatorStyles } from './execution-indicator.styles';

export class ExecutionIndicator extends BaseComponent {
  /** Контроллер состояния индикатора. */
  private readonly controller = new ExecutionIndicatorController();

  constructor() {
    super();
    // При изменении состояния обновляем только transform и подпись,
    // не пересоздавая DOM (чтобы сохранить CSS transition анимации).
    this.controller.setOnChange(() => this.applyPosition());
  }

  protected render(): void {
    this.update();
    // Применяем начальную позицию.
    this.applyPosition();
  }

  protected template(): string {
    return executionIndicatorTemplate(
      this.controller.getPosition(),
      this.controller.getLabel()
    );
  }

  protected styles(): string {
    return executionIndicatorStyles();
  }

  /** Устанавливает координаты позиций. */
  public setCoords(coords: Partial<Record<ExecutionPosition, PositionCoords>>): void {
    this.controller.setCoords(coords);
    this.applyPosition();
  }

  /** Перемещает индикатор в указанную позицию. */
  public moveTo(position: ExecutionPosition): void {
    this.controller.moveTo(position);
    this.applyPosition();
  }

  /** Сбрасывает индикатор в состояние ожидания. */
  public reset(): void {
    this.controller.reset();
    this.applyPosition();
  }

  /** Применяет координаты текущей позиции к элементу. */
  private applyPosition(): void {
    const root = this.query<HTMLElement>('.execution-indicator');
    if (!root) return;
    const coords = this.controller.getCoords(this.controller.getPosition());
    root.style.transform = `translate(${coords.x}px, ${coords.y}px)`;
    root.setAttribute('data-position', this.controller.getPosition());
    const label = this.query<HTMLElement>('.indicator-position');
    if (label) {
      label.textContent = this.controller.getLabel();
    }
  }
}

// Регистрация компонента
if (!customElements.get('execution-indicator')) {
  customElements.define('execution-indicator', ExecutionIndicator);
}