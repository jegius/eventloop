/**
 * Главный компонент визуализатора Event Loop.
 *
 * Оркестрирует все компоненты, обрабатывает события Event Loop
 * и управляет режимами работы (пошаговый / реальное время).
 *
 * Логика и состояние вынесены в контроллер, разметка — в темплейт,
 * стили — в отдельный файл. Сервисы внедряются через DI-контейнер.
 */

import { BaseComponent } from '../base-component';
import { container } from '../../di/container';
import { INTERPRETER_SERVICE, PLAYBACK_SERVICE } from '../../services/tokens';
import { EventLoopVisualizerController, type VisualizerComponents } from './event-loop-visualizer.controller';
import { eventLoopVisualizerTemplate } from './event-loop-visualizer.template';
import { eventLoopVisualizerStyles } from './event-loop-visualizer.styles';
import type { EventLoopEvent } from '../../types';
import { CallStack } from '../call-stack/call-stack';
import { QueueComponent } from '../queue/queue';
import { RenderPhases } from '../render-phases/render-phases';
import { ConsoleComponent } from '../console/console';
import { CodeEditor } from '../code-editor/code-editor';
import { ExecutionIndicator } from '../execution-indicator/execution-indicator';

export class EventLoopVisualizer extends BaseComponent {
  /** Контроллер визуализатора (лениво инициализируется). */
  private controller: EventLoopVisualizerController | null = null;

  constructor() {
    super();
  }

  /**
   * Лениво инициализирует контроллер.
   *
   * Сервисы резолвятся из DI-контейнера только при первом рендере,
   * когда они уже зарегистрированы в `main.ts`.
   */
  private getController(): EventLoopVisualizerController {
    if (!this.controller) {
      // Внедряем сервисы через DI-контейнер.
      const interpreter = container.resolve(INTERPRETER_SERVICE);
      const playback = container.resolve(PLAYBACK_SERVICE);
      this.controller = new EventLoopVisualizerController(interpreter, playback);

      // Обновляем кнопку воспроизведения при изменении состояния.
      this.controller.setOnPlayButtonChange(() => this.updatePlayButton());
    }
    return this.controller;
  }

  protected render(): void {
    this.update();
    this.captureComponents();
    this.bindEvents();
  }

  protected template(): string {
    return eventLoopVisualizerTemplate(
      this.getController().getMode(),
      this.getController().getSpeed(),
      this.getController().getPlayButtonLabel()
    );
  }

  protected styles(): string {
    return eventLoopVisualizerStyles();
  }

  /** Захватывает ссылки на дочерние компоненты. */
  private captureComponents(): void {
    const components: VisualizerComponents = {
      callStack: this.query<CallStack>('call-stack')!,
      microtaskQueue: this.query<QueueComponent>('#microtask-queue')!,
      macrotaskQueue: this.query<QueueComponent>('#macrotask-queue')!,
      renderQueue: this.query<QueueComponent>('#render-queue')!,
      renderPhases: this.query<RenderPhases>('render-phases')!,
      consoleComponent: this.query<ConsoleComponent>('event-loop-console')!,
      codeEditor: this.query<CodeEditor>('code-editor')!,
      executionIndicator: this.query<ExecutionIndicator>('execution-indicator')!,
    };
    this.getController().setComponents(components);

    // Колбэк добавления записи в журнал событий.
    this.getController().setLogHandler((event) => this.addLogEntry(event));

    // Настройка очередей откладывается, чтобы дочерние компоненты успели подключиться.
    requestAnimationFrame(() => this.getController().configureQueues());
  }

  /** Привязывает обработчики событий. */
  private bindEvents(): void {
    // Событие запуска кода из редактора
    this.addEventListener('run-code', this.handleRunCode as EventListener);

    // Кнопки управления
    this.query<HTMLButtonElement>('#play-btn')?.addEventListener('click', () => this.getController().togglePlay());
    this.query<HTMLButtonElement>('#step-btn')?.addEventListener('click', () => this.getController().stepForward());
    this.query<HTMLButtonElement>('#reset-btn')?.addEventListener('click', () => this.handleReset());

    // Переключение режима
    this.queryAll<HTMLButtonElement>('.mode-btn').forEach((btn) => {
      btn.addEventListener('click', () => {
        const mode = btn.dataset.mode as 'realtime' | 'step';
        this.getController().setMode(mode);
        // Обновляем активный класс кнопок
        this.queryAll<HTMLButtonElement>('.mode-btn').forEach((b) => {
          b.classList.toggle('active', b.dataset.mode === mode);
        });
        // Обновляем доступность кнопки "Шаг"
        const stepBtn = this.query<HTMLButtonElement>('#step-btn');
        if (stepBtn) {
          stepBtn.disabled = mode !== 'step';
        }
      });
    });

    // Скорость
    this.query<HTMLInputElement>('#speed-slider')?.addEventListener('input', (e) => {
      const speed = Number((e.target as HTMLInputElement).value);
      this.getController().setSpeed(speed);
      this.query<HTMLSpanElement>('.speed-value')!.textContent = `${speed}ms`;
    });
  }

  /** Обрабатывает событие запуска кода. */
  private handleRunCode = (event: CustomEvent): void => {
    const { code } = event.detail;
    this.getController().executeCode(code);
  };

  /** Обрабатывает сброс состояния. */
  private handleReset(): void {
    this.getController().reset();
    this.clearLog();
  }

  /** Добавляет запись в журнал событий. */
  private addLogEntry(event: EventLoopEvent): void {
    const logBody = this.query<HTMLElement>('#log-body');
    const count = this.query<HTMLElement>('#log-count');
    if (!logBody) return;

    const entry = document.createElement('div');
    entry.className = 'log-entry';
    entry.textContent = JSON.stringify(event);
    logBody.appendChild(entry);
    logBody.scrollTop = logBody.scrollHeight;

    if (count) {
      count.textContent = String(logBody.children.length);
    }
  }

  /** Очищает журнал событий. */
  private clearLog(): void {
    const logBody = this.query<HTMLElement>('#log-body');
    if (logBody) logBody.innerHTML = '';
    const count = this.query<HTMLElement>('#log-count');
    if (count) count.textContent = '0';
  }

  /** Обновляет подпись кнопки воспроизведения без пересоздания DOM. */
  private updatePlayButton(): void {
    const playBtn = this.query<HTMLButtonElement>('#play-btn');
    if (playBtn) {
      playBtn.textContent = this.getController().getPlayButtonLabel();
    }
  }
}

// Регистрация компонента
if (!customElements.get('event-loop-visualizer')) {
  customElements.define('event-loop-visualizer', EventLoopVisualizer);
}