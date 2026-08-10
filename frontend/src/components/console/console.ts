/**
 * Компонент консоли вывода.
 *
 * Отображает результаты выполнения JS-кода.
 * Логика и состояние вынесены в контроллер, разметка — в темплейт,
 * стили — в отдельный файл.
 */

import { BaseComponent } from '../base-component';
import { ConsoleController, type ConsoleEntry } from './console.controller';
import { consoleTemplate } from './console.template';
import { consoleStyles } from './console.styles';

export class ConsoleComponent extends BaseComponent {
  /** Контроллер состояния консоли. */
  private readonly controller = new ConsoleController();

  constructor() {
    super();
    // При изменении состояния перерисовываем компонент и прокручиваем вниз.
    this.controller.setOnChange(() => {
      this.update();
      this.scrollToBottom();
    });
  }

  protected render(): void {
    this.update();
  }

  protected template(): string {
    return consoleTemplate(this.controller.getEntries());
  }

  protected styles(): string {
    return consoleStyles();
  }

  /** Добавляет запись в консоль. */
  public log(text: string, type: ConsoleEntry['type'] = 'log'): void {
    this.controller.log(text, type);
  }

  /** Очищает консоль. */
  public clear(): void {
    this.controller.clear();
  }

  /** Прокручивает консоль вниз. */
  private scrollToBottom(): void {
    const body = this.query<HTMLElement>('#console-body');
    if (body) {
      body.scrollTop = body.scrollHeight;
    }
  }
}

// Регистрация компонента
if (!customElements.get('event-loop-console')) {
  customElements.define('event-loop-console', ConsoleComponent);
}