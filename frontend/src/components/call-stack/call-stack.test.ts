/**
 * Тесты для компонента CallStack.
 * Используется Vitest + jsdom.
 */

import { describe, it, expect, beforeEach } from 'vitest';
import { CallStack } from './call-stack';

// Регистрируем компонент, если ещё не зарегистрирован.
if (!customElements.get('call-stack')) {
  customElements.define('call-stack', CallStack);
}

describe('CallStack', () => {
  let stack: CallStack;

  beforeEach(() => {
    stack = document.createElement('call-stack') as CallStack;
    document.body.appendChild(stack);
  });

  it('добавляет элемент в стек', () => {
    stack.push('main');
    stack.push('foo');
    const items = stack.shadowRoot?.querySelectorAll('.stack-item');
    expect(items?.length).toBe(2);
  });

  it('удаляет верхний элемент стека (LIFO)', () => {
    stack.push('main');
    stack.push('foo');
    stack.pop();
    const items = stack.shadowRoot?.querySelectorAll('.stack-item');
    expect(items?.length).toBe(1);
    expect(items?.[0]?.textContent).toContain('main');
  });

  it('очищает стек', () => {
    stack.push('main');
    stack.clear();
    const items = stack.shadowRoot?.querySelectorAll('.stack-item');
    expect(items?.length).toBe(0);
  });

  it('отображает пустое состояние', () => {
    const empty = stack.shadowRoot?.querySelector('.empty');
    expect(empty?.textContent).toContain('Стек пуст');
  });
});