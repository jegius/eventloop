/**
 * Универсальный компонент очереди Event Loop.
 *
 * Используется для Microtask, Macrotask, Render, rAF, rIC.
 * Логика и состояние вынесены в контроллер, разметка — в темплейт,
 * стили — в отдельный файл.
 */

import { BaseComponent } from '../base-component';
import { QueueController, type QueueConfig } from './queue.controller';
import { queueTemplate } from './queue.template';
import { queueStyles } from './queue.styles';
import type { QueueItem } from '../../types';

export class QueueComponent extends BaseComponent {
  /** Контроллер состояния очереди. */
  private readonly controller = new QueueController();

  constructor() {
    super();
    // При изменении состояния перерисовываем компонент.
    this.controller.setOnChange(() => this.update());
  }

  /** Устанавливает конфигурацию очереди. */
  public setConfig(config: QueueConfig): void {
    this.controller.setConfig(config);
  }

  protected render(): void {
    this.update();
  }

  protected template(): string {
    return queueTemplate(this.controller.getConfig(), this.controller.getItems());
  }

  protected styles(): string {
    return queueStyles();
  }

  /** Добавляет элемент в очередь. */
  public enqueue(label: string): void {
    this.controller.enqueue(label);
  }

  /** Извлекает первый элемент из очереди. */
  public dequeue(): QueueItem | undefined {
    return this.controller.dequeue();
  }

  /** Очищает очередь. */
  public clear(): void {
    this.controller.clear();
  }

  /** Возвращает количество элементов. */
  public size(): number {
    return this.controller.size();
  }
}

// Регистрация компонента
if (!customElements.get('event-loop-queue')) {
  customElements.define('event-loop-queue', QueueComponent);
}