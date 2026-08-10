/**
 * Контроллер компонента индикатора потока исполнения.
 *
 * Содержит состояние (текущая позиция потока и координаты позиций)
 * и логику перемещения индикатора между очередями Event Loop.
 * Не зависит от DOM — рендеринг выполняется через колбэк `onChange`.
 */

/** Позиции, где может находиться поток исполнения. */
export type ExecutionPosition =
  | 'idle'      // Поток не активен (до запуска / после сброса).
  | 'callstack' // Поток в Call Stack (выполняется синхронный код).
  | 'microtask' // Поток в Microtask Queue.
  | 'macrotask' // Поток в Macrotask Queue.
  | 'render'    // Поток в Render Queue / фазе рендера.
  | 'ric';      // Поток в Idle Callback (rIC).

/** Подпись позиции для отображения. */
export const POSITION_LABELS: Record<ExecutionPosition, string> = {
  idle: 'Ожидание',
  callstack: 'Call Stack',
  microtask: 'Microtask',
  macrotask: 'Macrotask',
  render: 'Render',
  ric: 'Idle Callback',
};

/** Координаты позиции (относительно контейнера индикатора). */
export interface PositionCoords {
  x: number;
  y: number;
}

/**
 * Контроллер индикатора потока исполнения.
 *
 * Реализует паттерн Controller: управляет состоянием и логикой перемещения
 * индикатора, уведомляя представление (компонент) об изменениях через колбэк.
 */
export class ExecutionIndicatorController {
  /** Текущая позиция потока. */
  private position: ExecutionPosition = 'idle';

  /** Координаты всех позиций (относительно контейнера). */
  private coords: Record<ExecutionPosition, PositionCoords> = {
    idle: { x: 0, y: 0 },
    callstack: { x: 0, y: 0 },
    microtask: { x: 0, y: 0 },
    macrotask: { x: 0, y: 0 },
    render: { x: 0, y: 0 },
    ric: { x: 0, y: 0 },
  };

  /** Колбэк уведомления об изменении состояния. */
  private onChange: (() => void) | null = null;

  /** Устанавливает колбэк уведомления об изменении. */
  setOnChange(callback: () => void): void {
    this.onChange = callback;
  }

  /** Возвращает текущую позицию потока. */
  getPosition(): ExecutionPosition {
    return this.position;
  }

  /** Возвращает подпись текущей позиции. */
  getLabel(): string {
    return POSITION_LABELS[this.position];
  }

  /** Возвращает координаты указанной позиции. */
  getCoords(position: ExecutionPosition): PositionCoords {
    return this.coords[position];
  }

  /** Устанавливает координаты позиций. */
  setCoords(coords: Partial<Record<ExecutionPosition, PositionCoords>>): void {
    this.coords = { ...this.coords, ...coords };
    this.notify();
  }

  /** Перемещает индикатор в указанную позицию. */
  moveTo(position: ExecutionPosition): void {
    if (this.position === position) return;
    this.position = position;
    this.notify();
  }

  /** Сбрасывает индикатор в состояние ожидания. */
  reset(): void {
    this.moveTo('idle');
  }

  /** Уведомляет представление об изменении состояния. */
  private notify(): void {
    this.onChange?.();
  }
}