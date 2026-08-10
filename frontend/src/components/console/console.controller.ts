/**
 * Контроллер компонента консоли вывода.
 *
 * Содержит состояние (записи консоли) и бизнес-логику управления
 * записями. Не зависит от DOM — рендеринг выполняется через колбэк
 * `onChange`.
 */

/** Запись в консоли. */
export interface ConsoleEntry {
  text: string;
  type: 'log' | 'error' | 'info' | 'warning';
}

/**
 * Контроллер консоли.
 *
 * Реализует паттерн Controller: управляет состоянием и логикой записей,
 * уведомляя представление (компонент) об изменениях через колбэк.
 */
export class ConsoleController {
  /** Записи консоли. */
  private entries: ConsoleEntry[] = [];

  /** Колбэк уведомления об изменении состояния. */
  private onChange: (() => void) | null = null;

  /** Устанавливает колбэк уведомления об изменении. */
  setOnChange(callback: () => void): void {
    this.onChange = callback;
  }

  /** Возвращает записи консоли. */
  getEntries(): ConsoleEntry[] {
    return this.entries;
  }

  /** Добавляет запись в консоль. */
  log(text: string, type: ConsoleEntry['type'] = 'log'): void {
    this.entries.push({ text, type });
    this.notify();
  }

  /** Очищает консоль. */
  clear(): void {
    this.entries = [];
    this.notify();
  }

  /** Уведомляет представление об изменении состояния. */
  private notify(): void {
    this.onChange?.();
  }
}