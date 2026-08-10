/**
 * Контроллер компонента редактора кода.
 *
 * Содержит состояние (текущий код) и базовую логику управления кодом.
 * Не зависит от DOM — рендеринг и подсветка выполняются компонентом.
 */

/**
 * Контроллер редактора кода.
 *
 * Реализует паттерн Controller: управляет состоянием кода,
 * уведомляя представление (компонент) об изменениях через колбэк.
 */
export class CodeEditorController {
  /** Текущий код. */
  private code: string = '';

  /** Колбэк уведомления об изменении состояния. */
  private onChange: (() => void) | null = null;

  /** Устанавливает колбэк уведомления об изменении. */
  setOnChange(callback: () => void): void {
    this.onChange = callback;
  }

  /** Возвращает текущий код. */
  getCode(): string {
    return this.code;
  }

  /** Устанавливает текущий код. */
  setCode(code: string): void {
    this.code = code;
    this.notify();
  }

  /** Очищает код. */
  clear(): void {
    this.code = '';
    this.notify();
  }

  /** Уведомляет представление об изменении состояния. */
  private notify(): void {
    this.onChange?.();
  }
}