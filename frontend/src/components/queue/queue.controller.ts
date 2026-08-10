/**
 * Контроллер компонента очереди Event Loop.
 *
 * Содержит состояние (конфигурация и элементы очереди) и бизнес-логику
 * управления очередью (enqueue / dequeue / clear). Не зависит от DOM —
 * рендеринг выполняется через колбэк `onChange`.
 */

import type { QueueItem } from '../../types';

/** Параметры очереди. */
export interface QueueConfig {
  /** Название очереди. */
  title: string;
  /** Цвет очереди. */
  color: string;
  /** Описание очереди. */
  description?: string;
}

/**
 * Контроллер очереди.
 *
 * Реализует паттерн Controller: управляет состоянием и логикой очереди,
 * уведомляя представление (компонент) об изменениях через колбэк.
 */
export class QueueController {
  /** Конфигурация очереди. */
  private config: QueueConfig = { title: 'Queue', color: '#4f8cff' };

  /** Элементы очереди. */
  private items: QueueItem[] = [];

  /** Колбэк уведомления об изменении состояния. */
  private onChange: (() => void) | null = null;

  /** Устанавливает колбэк уведомления об изменении. */
  setOnChange(callback: () => void): void {
    this.onChange = callback;
  }

  /** Устанавливает конфигурацию очереди. */
  setConfig(config: QueueConfig): void {
    this.config = config;
    this.notify();
  }

  /** Возвращает конфигурацию очереди. */
  getConfig(): QueueConfig {
    return this.config;
  }

  /** Возвращает элементы очереди. */
  getItems(): QueueItem[] {
    return this.items;
  }

  /** Добавляет элемент в очередь. */
  enqueue(label: string): void {
    const id = this.items.length + 1;
    this.items.push({ id, label, color: this.config.color });
    this.notify();
  }

  /** Извлекает первый элемент из очереди. */
  dequeue(): QueueItem | undefined {
    const item = this.items.shift();
    this.notify();
    return item;
  }

  /** Очищает очередь. */
  clear(): void {
    this.items = [];
    this.notify();
  }

  /** Возвращает количество элементов. */
  size(): number {
    return this.items.length;
  }

  /** Уведомляет представление об изменении состояния. */
  private notify(): void {
    this.onChange?.();
  }
}