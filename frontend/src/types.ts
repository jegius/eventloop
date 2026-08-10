/**
 * Типы данных для визуализатора Event Loop.
 * Эти типы соответствуют событиям, генерируемым Rust-интерпретатором.
 */

/** Фаза рендера. */
export type RenderPhase =
  | 'AnimationFrame'
  | 'ParsingHtml'
  | 'ParsingCss'
  | 'Dom'
  | 'Cssom'
  | 'RenderTree'
  | 'Layout'
  | 'Paint'
  | 'Compose';

/** Событие Event Loop (соответствует EventLoopEvent в Rust). */
export type EventLoopEvent =
  | { type: 'SyncCode'; message: string }
  | { type: 'CallStackPush'; label: string; line?: number; endLine?: number }
  | { type: 'CallStackPop'; label: string }
  | { type: 'MicrotaskEnqueue'; label: string }
  | { type: 'MicrotaskDequeue'; label: string }
  | { type: 'MacrotaskEnqueue'; label: string }
  | { type: 'MacrotaskDequeue'; label: string }
  | { type: 'RafEnqueue'; label: string }
  | { type: 'RafDequeue'; label: string }
  | { type: 'RicEnqueue'; label: string }
  | { type: 'RicDequeue'; label: string }
  | { type: 'RenderRequested' }
  | { type: 'RenderPhase'; phase: RenderPhase }
  | { type: 'ConsoleLog'; message: string }
  | { type: 'Warning'; message: string };

/** Результат выполнения кода. */
export interface RunResult {
  /** События Event Loop (в JSON-формате). */
  events: string[];
  /** Вывод консоли. */
  consoleOutput: string[];
}

/** Режим работы визуализатора. */
export type PlaybackMode = 'step' | 'realtime';

/** Состояние выполнения. */
export type ExecutionState = 'idle' | 'running' | 'paused' | 'done';

/** Элемент в очереди (для визуализации). */
export interface QueueItem {
  id: number;
  label: string;
  color: string;
}

/** Элемент Call Stack. */
export interface StackItem {
  label: string;
}