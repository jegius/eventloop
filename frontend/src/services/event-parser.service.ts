/**
 * Сервис парсинга событий Event Loop.
 *
 * Преобразует строковые представления событий, генерируемых
 * Rust-интерпретатором (например, `CallStackPush("main", 5, 8)`),
 * в типизированные объекты `EventLoopEvent`.
 *
 * Внедряется через DI-контейнер.
 */

import type { EventLoopEvent } from '../types';

/**
 * Сервис парсинга событий Event Loop.
 *
 * Реализует паттерн Service: инкапсулирует логику разбора строковых
 * событий интерпретатора в структурированные объекты.
 */
export class EventParserService {
  /**
   * Парсит строку события в объект `EventLoopEvent`.
   *
   * @param eventStr Строковое представление события.
   * @returns Типизированный объект события.
   */
  parse(eventStr: string): EventLoopEvent {
    // События без аргументов (без скобок), например "RenderRequested".
    if (eventStr === 'RenderRequested') {
      return { type: 'RenderRequested' };
    }

    // Простой парсер для событий вида "CallStackPush(\"main\")"
    const match = eventStr.match(/^(\w+)\((.*)\)$/s);
    if (!match) {
      return { type: 'SyncCode', message: eventStr };
    }
    const type = match[1] as EventLoopEvent['type'];
    const arg = match[2].replace(/^"|"$/g, '');

    switch (type) {
      case 'CallStackPush': {
        // Событие вида CallStackPush("label", 5, 8). Второй и третий аргументы —
        // диапазон строк (start, end), покрывающий весь колбэк.
        const inner = match[2];
        const labelMatch = inner.match(/^"((?:[^"\\]|\\.)*)"/);
        const lineMatch = inner.match(/,\s*(\d+)\s*,\s*(\d+)\s*$/);
        const label = labelMatch ? labelMatch[1] : arg;
        const line = lineMatch ? Number(lineMatch[1]) : undefined;
        const endLine = lineMatch ? Number(lineMatch[2]) : undefined;
        return { type, label, line, endLine } as EventLoopEvent;
      }
      case 'CallStackPop':
      case 'MicrotaskEnqueue':
      case 'MicrotaskDequeue':
      case 'MacrotaskEnqueue':
      case 'MacrotaskDequeue':
      case 'RafEnqueue':
      case 'RafDequeue':
      case 'RicEnqueue':
      case 'RicDequeue':
        return { type, label: arg } as EventLoopEvent;
      case 'RenderPhase':
        return { type, phase: arg as EventLoopEvent extends { type: 'RenderPhase' } ? EventLoopEvent['phase'] : never };
      case 'ConsoleLog':
        return { type, message: arg };
      case 'SyncCode':
        return { type, message: arg };
      case 'RenderRequested':
        return { type };
      case 'Warning':
        return { type, message: arg };
      default:
        return { type: 'SyncCode', message: eventStr };
    }
  }

  /**
   * Парсит массив строковых событий в массив объектов.
   *
   * @param events Массив строковых событий.
   * @returns Массив типизированных событий.
   */
  parseAll(events: string[]): EventLoopEvent[] {
    return events.map((e) => this.parse(e));
  }
}