/**
 * Тесты для компонента QueueComponent.
 * Используется Vitest + jsdom.
 */

import { describe, it, expect, beforeEach } from 'vitest';
import { QueueComponent } from './queue';

// Регистрируем компонент, если ещё не зарегистрирован.
if (!customElements.get('event-loop-queue')) {
  customElements.define('event-loop-queue', QueueComponent);
}

describe('QueueComponent', () => {
  let queue: QueueComponent;

  beforeEach(() => {
    queue = document.createElement('event-loop-queue') as QueueComponent;
    document.body.appendChild(queue);
    queue.setConfig({ title: 'Microtask', color: '#ff5c5c' });
  });

  it('добавляет элемент в очередь', () => {
    queue.enqueue('task1');
    expect(queue.size()).toBe(1);
  });

  it('извлекает первый элемент из очереди (FIFO)', () => {
    queue.enqueue('task1');
    queue.enqueue('task2');
    const item = queue.dequeue();
    expect(item?.label).toBe('task1');
    expect(queue.size()).toBe(1);
  });

  it('очищает очередь', () => {
    queue.enqueue('task1');
    queue.enqueue('task2');
    queue.clear();
    expect(queue.size()).toBe(0);
  });

  it('отображает количество элементов', () => {
    queue.enqueue('task1');
    queue.enqueue('task2');
    const count = queue.shadowRoot?.querySelector('.count');
    expect(count?.textContent).toBe('2');
  });
});