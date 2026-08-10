/**
 * Сервис управления воспроизведением событий Event Loop.
 *
 * Инкапсулирует логику воспроизведения последовательности событий:
 * управление таймером реального времени, пошаговое выполнение,
 * состояние выполнения (idle / running / paused / done).
 *
 * Сервис не зависит от конкретных компонентов: обработка каждого события
 * делегируется через колбэк `onEvent`, который предоставляет потребитель.
 *
 * Внедряется через DI-контейнер.
 */

import type { EventLoopEvent, ExecutionState, PlaybackMode } from '../types';
import type { EventParserService } from './event-parser.service';

/** Колбэк обработки одного события. */
export type EventHandler = (event: EventLoopEvent) => void;

/**
 * Сервис управления воспроизведением.
 *
 * Реализует паттерн Service: инкапсулирует бизнес-логику воспроизведения
 * событий и предоставляет её компонентам через DI.
 */
export class PlaybackService {
  /** Состояние выполнения. */
  private state: ExecutionState = 'idle';

  /** Режим работы. */
  private mode: PlaybackMode = 'realtime';

  /** Скорость выполнения (мс на шаг). */
  private speed: number = 500;

  /** События Event Loop. */
  private events: EventLoopEvent[] = [];

  /** Текущий индекс события. */
  private currentEventIndex: number = 0;

  /** Таймер для режима реального времени. */
  private timer: number | null = null;

  /** Колбэк обработки события. */
  private onEvent: EventHandler = () => {};

  /** Колбэк завершения воспроизведения. */
  private onComplete: (() => void) | null = null;

  constructor(private readonly eventParser: EventParserService) {}

  /** Возвращает текущее состояние выполнения. */
  getState(): ExecutionState {
    return this.state;
  }

  /** Возвращает текущий режим работы. */
  getMode(): PlaybackMode {
    return this.mode;
  }

  /** Возвращает текущую скорость (мс на шаг). */
  getSpeed(): number {
    return this.speed;
  }

  /** Возвращает количество событий. */
  getEventCount(): number {
    return this.events.length;
  }

  /** Возвращает текущий индекс события. */
  getCurrentIndex(): number {
    return this.currentEventIndex;
  }

  /**
   * Устанавливает колбэк обработки события.
   */
  setEventHandler(handler: EventHandler): void {
    this.onEvent = handler;
  }

  /**
   * Устанавливает колбэк завершения воспроизведения.
   */
  setOnComplete(callback: () => void): void {
    this.onComplete = callback;
  }

  /**
   * Устанавливает режим работы.
   */
  setMode(mode: PlaybackMode): void {
    this.mode = mode;
  }

  /**
   * Устанавливает скорость выполнения.
   */
  setSpeed(speed: number): void {
    this.speed = speed;
  }

  /**
   * Загружает события для воспроизведения.
   *
   * @param rawEvents Строковые события от интерпретатора.
   */
  loadEvents(rawEvents: string[]): void {
    this.events = this.eventParser.parseAll(rawEvents);
    this.currentEventIndex = 0;
  }

  /**
   * Запускает воспроизведение в реальном времени.
   * Если событий нет, сразу завершает.
   */
  startRealtime(): void {
    this.stopTimer();
    if (this.events.length === 0) {
      this.finish();
      return;
    }
    this.state = 'running';
    this.timer = window.setInterval(() => {
      if (this.currentEventIndex >= this.events.length) {
        this.finish();
        return;
      }
      this.processCurrent();
      this.currentEventIndex++;
    }, this.speed);
  }

  /**
   * Переключает паузу/воспроизведение.
   */
  togglePlay(): void {
    if (this.state === 'running') {
      this.stopTimer();
      this.state = 'paused';
    } else if (this.state === 'paused' || this.state === 'done') {
      this.state = 'running';
      if (this.mode === 'realtime') {
        this.startRealtime();
      }
    }
  }

  /**
   * Устанавливает состояние "paused" (для пошагового режима).
   */
  pause(): void {
    this.stopTimer();
    this.state = 'paused';
  }

  /**
   * Делает один шаг вперёд (пошаговый режим).
   */
  stepForward(): void {
    if (this.currentEventIndex < this.events.length) {
      this.processCurrent();
      this.currentEventIndex++;
    } else {
      this.state = 'done';
    }
  }

  /**
   * Сбрасывает состояние воспроизведения.
   */
  reset(): void {
    this.stopTimer();
    this.events = [];
    this.currentEventIndex = 0;
    this.state = 'idle';
  }

  /**
   * Останавливает таймер.
   */
  stop(): void {
    this.stopTimer();
  }

  /** Обрабатывает текущее событие через колбэк. */
  private processCurrent(): void {
    const event = this.events[this.currentEventIndex];
    if (event) {
      this.onEvent(event);
    }
  }

  /** Завершает воспроизведение. */
  private finish(): void {
    this.stopTimer();
    this.state = 'done';
    this.onComplete?.();
  }

  /** Останавливает таймер. */
  private stopTimer(): void {
    if (this.timer !== null) {
      window.clearInterval(this.timer);
      this.timer = null;
    }
  }
}