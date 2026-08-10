/**
 * Контроллер главного компонента визуализатора Event Loop.
 *
 * Оркестрирует все дочерние компоненты, обрабатывает события Event Loop
 * и управляет режимами работы (пошаговый / реальное время).
 *
 * Использует сервисы через DI-контейнер: InterpreterService (выполнение
 * кода) и PlaybackService (воспроизведение событий).
 */

import type { EventLoopEvent, PlaybackMode } from '../../types';
import type { InterpreterService } from '../../services/interpreter.service';
import type { PlaybackService } from '../../services/playback.service';
import type { CallStack } from '../call-stack/call-stack';
import type { QueueComponent } from '../queue/queue';
import type { RenderPhases } from '../render-phases/render-phases';
import type { ConsoleComponent } from '../console/console';
import type { CodeEditor } from '../code-editor/code-editor';
import type { ExecutionIndicator } from '../execution-indicator/execution-indicator';

/** Порт для доступа к дочерним компонентам. */
export interface VisualizerComponents {
  callStack: CallStack;
  microtaskQueue: QueueComponent;
  macrotaskQueue: QueueComponent;
  renderQueue: QueueComponent;
  renderPhases: RenderPhases;
  consoleComponent: ConsoleComponent;
  codeEditor: CodeEditor;
  executionIndicator: ExecutionIndicator;
}

/** Колбэк добавления записи в журнал событий. */
export type LogHandler = (event: EventLoopEvent) => void;

/**
 * Контроллер визуализатора.
 *
 * Реализует паттерн Controller: управляет состоянием и логикой
 * оркестрации, делегируя выполнение кода и воспроизведение сервисам.
 */
export class EventLoopVisualizerController {
  /** Режим работы. */
  private mode: PlaybackMode = 'realtime';

  /** Ссылки на дочерние компоненты. */
  private components: VisualizerComponents | null = null;

  /** Колбэк добавления записи в журнал событий. */
  private logHandler: LogHandler | null = null;

  /** Колбэк обновления подписи кнопки воспроизведения. */
  private onPlayButtonChange: (() => void) | null = null;

  constructor(
    private readonly interpreter: InterpreterService,
    private readonly playback: PlaybackService
  ) {
    // Обработка каждого события делегируется компонентам.
    this.playback.setEventHandler((event) => this.processEvent(event));
    // При завершении воспроизведения обновляем кнопку.
    this.playback.setOnComplete(() => this.onPlayButtonChange?.());
  }

  /** Устанавливает ссылки на дочерние компоненты. */
  setComponents(components: VisualizerComponents): void {
    this.components = components;
  }

  /** Устанавливает колбэк добавления записи в журнал. */
  setLogHandler(handler: LogHandler): void {
    this.logHandler = handler;
  }

  /** Устанавливает колбэк обновления кнопки воспроизведения. */
  setOnPlayButtonChange(callback: () => void): void {
    this.onPlayButtonChange = callback;
  }

  /** Возвращает текущий режим работы. */
  getMode(): PlaybackMode {
    return this.mode;
  }

  /** Возвращает текущую скорость. */
  getSpeed(): number {
    return this.playback.getSpeed();
  }

  /** Возвращает подпись кнопки воспроизведения. */
  getPlayButtonLabel(): string {
    switch (this.playback.getState()) {
      case 'running':
        return '⏸ Пауза';
      case 'paused':
        return '▶ Продолжить';
      default:
        return '▶ Запуск';
    }
  }

  /** Устанавливает режим работы. */
  setMode(mode: PlaybackMode): void {
    this.mode = mode;
    this.playback.setMode(mode);
  }

  /** Устанавливает скорость выполнения. */
  setSpeed(speed: number): void {
    this.playback.setSpeed(speed);
  }

  /** Настраивает очереди. */
  configureQueues(): void {
    this.components?.microtaskQueue?.setConfig({
      title: 'Microtask Queue',
      color: 'var(--color-microtask)',
      description: 'Promise, queueMicrotask',
    });
    this.components?.macrotaskQueue?.setConfig({
      title: 'Macrotask Queue',
      color: 'var(--color-macrotask)',
      description: 'setTimeout, setInterval, I/O, fetch',
    });
    this.components?.renderQueue?.setConfig({
      title: 'Render Queue',
      color: 'var(--color-render)',
      description: 'rAF, rIC, render',
    });

    // Вычисляем координаты позиций для индикатора потока исполнения.
    this.computeIndicatorCoords();
  }

  /** Вычисляет координаты позиций индикатора относительно его контейнера. */
  private computeIndicatorCoords(): void {
    const c = this.components;
    if (!c) return;

    const container = c.executionIndicator as unknown as HTMLElement;
    const containerRect = container.getBoundingClientRect();

    const getCenter = (el: HTMLElement) => {
      const rect = el.getBoundingClientRect();
      return {
        x: rect.left + rect.width / 2 - containerRect.left,
        y: rect.top + rect.height / 2 - containerRect.top,
      };
    };

    c.executionIndicator.setCoords({
      callstack: getCenter(c.callStack as unknown as HTMLElement),
      microtask: getCenter(c.microtaskQueue as unknown as HTMLElement),
      macrotask: getCenter(c.macrotaskQueue as unknown as HTMLElement),
      render: getCenter(c.renderQueue as unknown as HTMLElement),
      ric: getCenter(c.renderQueue as unknown as HTMLElement),
    });
  }

  /** Выполняет JS-код через интерпретатор. */
  async executeCode(code: string): Promise<void> {
    this.reset();
    this.playback.setMode(this.mode);

    try {
      const result = await this.interpreter.run(code);

      // Загружаем события в сервис воспроизведения.
      this.playback.loadEvents(result.events);

      // Консоль заполняется пошагово через события ConsoleLog.
      // Если событий нет (например, ошибка парсинга), выводим консоль сразу.
      if (result.events.length === 0) {
        result.consoleOutput.forEach((line) => this.components?.consoleComponent.log(line));
      }

      if (this.mode === 'realtime') {
        this.playback.startRealtime();
      } else {
        // В пошаговом режиме останавливаемся в состоянии "paused".
        this.playback.pause();
      }
      this.onPlayButtonChange?.();
    } catch (error) {
      this.components?.consoleComponent.log(`Ошибка: ${error}`, 'error');
      this.onPlayButtonChange?.();
    }
  }

  /** Обрабатывает одно событие. */
  private processEvent(event: EventLoopEvent): void {
    const c = this.components;
    if (!c) return;

    switch (event.type) {
      case 'CallStackPush':
        c.callStack.push(event.label);
        // Поток исполнения находится в Call Stack.
        c.executionIndicator?.moveTo('callstack');
        // Подсвечиваем весь колбэк (диапазон строк) в редакторе кода.
        if (event.line) {
          c.codeEditor?.highlightLineRange(event.line, event.endLine ?? event.line);
        }
        break;
      case 'CallStackPop':
        c.callStack.pop();
        break;
      case 'MicrotaskEnqueue':
        // Задача распределяется из call stack в очередь.
        c.callStack.highlight();
        c.microtaskQueue.enqueue(event.label);
        break;
      case 'MicrotaskDequeue':
        // Поток исполнения извлекает задачу из Microtask Queue.
        c.executionIndicator?.moveTo('microtask');
        c.microtaskQueue.dequeue();
        break;
      case 'MacrotaskEnqueue':
        // Задача распределяется из call stack в очередь.
        c.callStack.highlight();
        c.macrotaskQueue.enqueue(event.label);
        break;
      case 'MacrotaskDequeue':
        // Поток исполнения извлекает задачу из Macrotask Queue.
        c.executionIndicator?.moveTo('macrotask');
        c.macrotaskQueue.dequeue();
        break;
      case 'RafEnqueue':
        // rAF — часть шага рендеринга, попадает в Render Queue.
        c.callStack.highlight();
        c.renderQueue.enqueue(event.label);
        break;
      case 'RafDequeue':
        // Поток исполнения извлекает rAF из Render Queue.
        c.executionIndicator?.moveTo('render');
        // rAF удаляется из Render Queue.
        c.renderQueue.dequeue();
        break;
      case 'RicEnqueue':
        // rIC — часть шага рендеринга, попадает в Render Queue.
        c.callStack.highlight();
        c.renderQueue.enqueue(event.label);
        break;
      case 'RicDequeue':
        // Поток исполнения выполняет rIC (idle callback).
        c.executionIndicator?.moveTo('ric');
        // rIC удаляется из Render Queue.
        c.renderQueue.dequeue();
        break;
      case 'RenderRequested':
        c.renderQueue.enqueue('render');
        break;
      case 'RenderPhase':
        // Поток исполнения находится в фазе рендера.
        c.executionIndicator?.moveTo('render');
        c.renderPhases.setActivePhase(event.phase);
        break;
      case 'ConsoleLog':
        c.consoleComponent.log(event.message);
        break;
      case 'SyncCode':
        // Внутреннее событие — не выводим в консоль, только в Event Log
        break;
      case 'Warning':
        // Статический анализ обнаружил потенциальное зацикливание очередей.
        // Выводим предупреждение в консоль с рекомендацией прервать исполнение
        // кнопкой «Сброс», если очередь начнёт бесконечно расти. Воспроизведение
        // НЕ останавливается автоматически — пользователь сам решает, прервать ли.
        c.consoleComponent.log(event.message, 'warning');
        break;
    }

    this.logHandler?.(event);
  }

  /** Переключает паузу/воспроизведение. */
  togglePlay(): void {
    this.playback.togglePlay();
    this.onPlayButtonChange?.();
  }

  /** Делает один шаг вперёд (пошаговый режим). */
  stepForward(): void {
    this.playback.stepForward();
    this.onPlayButtonChange?.();
  }

  /** Сбрасывает состояние. */
  reset(): void {
    this.playback.reset();
    this.components?.callStack?.clear();
    this.components?.microtaskQueue?.clear();
    this.components?.macrotaskQueue?.clear();
    this.components?.renderQueue?.clear();
    this.components?.renderPhases?.reset();
    this.components?.executionIndicator?.reset();
    this.onPlayButtonChange?.();
  }
}