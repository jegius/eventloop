/**
 * Базовый класс для всех Web Components визуализатора.
 * Предоставляет общую логику: Shadow DOM, стили, рендеринг.
 */

export abstract class BaseComponent extends HTMLElement {
  protected shadow: ShadowRoot;

  constructor() {
    super();
    this.shadow = this.attachShadow({ mode: 'open' });
  }

  /**
   * Вызывается при подключении компонента к DOM.
   * Рендерит содержимое и стили.
   */
  connectedCallback(): void {
    this.render();
  }

  /**
   * Рендерит содержимое компонента.
   * Должен быть переопределён в подклассах.
   */
  protected abstract render(): void;

  /**
   * Возвращает HTML-содержимое компонента.
   * Должен быть переопределён в подклассах.
   */
  protected abstract template(): string;

  /**
   * Возвращает CSS-стили компонента.
   * Может быть переопределён в подклассах.
   */
  protected styles(): string {
    return '';
  }

  /** Обновляет содержимое Shadow DOM. */
  protected update(): void {
    this.shadow.innerHTML = `
      <style>${this.styles()}</style>
      ${this.template()}
    `;
  }

  /** Находит элемент внутри Shadow DOM. */
  protected query<T extends HTMLElement>(selector: string): T | null {
    return this.shadow.querySelector<T>(selector);
  }

  /** Находит все элементы внутри Shadow DOM. */
  protected queryAll<T extends HTMLElement>(selector: string): NodeListOf<T> {
    return this.shadow.querySelectorAll<T>(selector);
  }
}