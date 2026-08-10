/**
 * Контроллер компонента Call Stack.
 *
 * Содержит состояние (элементы стека) и бизнес-логику управления стеком.
 * Не зависит от DOM — рендеринг выполняется через колбэк `onChange`,
 * который предоставляет компонент.
 */

import type { StackItem } from '../../types';

/**
 * Контроллер Call Stack.
 *
 * Реализует паттерн Controller: управляет состоянием и логикой стека,
 * уведомляя представление (компонент) об изменениях через колбэк.
 */
export class CallStackController {
  /** Элементы стека. */
  private items: StackItem[] = [];

  /** Колбэк уведомления об изменении состояния. */
  private onChange: (() => void) | null = null;

  /** Устанавливает колбэк уведомления об изменении. */
  setOnChange(callback: () => void): void {
    this.onChange = callback;
  }

  /** Возвращает элементы стека. */
  getItems(): StackItem[] {
    return this.items;
  }

  /** Добавляет элемент в стек. */
  push(label: string): void {
    this.items.push({ label });
    this.notify();
  }

  /** Удаляет верхний элемент стека. */
  pop(): void {
    this.items.pop();
    this.notify();
  }

  /** Очищает стек. */
  clear(): void {
    this.items = [];
    this.notify();
  }

  /** Уведомляет представление об изменении состояния. */
  private notify(): void {
    this.onChange?.();
  }
}