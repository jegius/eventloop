/**
 * Компонент визуализации Call Stack.
 *
 * Отображает стек вызовов функций. Логика и состояние вынесены
 * в контроллер, разметка — в темплейт, стили — в отдельный файл.
 */

import { BaseComponent } from '../base-component';
import { CallStackController } from './call-stack.controller';
import { callStackTemplate } from './call-stack.template';
import { callStackStyles } from './call-stack.styles';

export class CallStack extends BaseComponent {
  /** Контроллер состояния стека. */
  private readonly controller = new CallStackController();

  constructor() {
    super();
    // При изменении состояния перерисовываем компонент.
    this.controller.setOnChange(() => this.update());
  }

  protected render(): void {
    this.update();
  }

  protected template(): string {
    return callStackTemplate(this.controller.getItems());
  }

  protected styles(): string {
    return callStackStyles();
  }

  /** Добавляет элемент в стек. */
  public push(label: string): void {
    this.controller.push(label);
  }

  /** Удаляет верхний элемент стека. */
  public pop(): void {
    this.controller.pop();
  }

  /** Очищает стек. */
  public clear(): void {
    this.controller.clear();
  }

  /** Подсвечивает стек (при распределении задач по очередям). */
  public highlight(): void {
    const body = this.shadowRoot?.querySelector('.stack-body');
    if (!body) return;
    body.classList.add('distributing');
    // Снимаем подсветку через короткое время.
    setTimeout(() => body.classList.remove('distributing'), 300);
  }
}

// Регистрация компонента
if (!customElements.get('call-stack')) {
  customElements.define('call-stack', CallStack);
}