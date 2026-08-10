/**
 * Токены сервисов для DI-контейнера.
 *
 * Токены используются как ключи при регистрации и резолве сервисов
 * в DI-контейнере. Вынесены в отдельный файл, чтобы избежать циклических
 * зависимостей между сервисами и компонентами.
 */

import type { Container, ServiceToken } from '../di/container';
import { InterpreterService } from './interpreter.service';
import { EventParserService } from './event-parser.service';
import { PlaybackService } from './playback.service';

/** Токен сервиса интерпретатора (обёртка над WASM). */
export const INTERPRETER_SERVICE: ServiceToken<InterpreterService> = 'interpreter';

/** Токен сервиса парсинга событий Event Loop. */
export const EVENT_PARSER_SERVICE: ServiceToken<EventParserService> = 'event-parser';

/** Токен сервиса управления воспроизведением. */
export const PLAYBACK_SERVICE: ServiceToken<PlaybackService> = 'playback';

/**
 * Регистрирует все сервисы приложения в DI-контейнере.
 * Вызывается один раз при инициализации приложения (в `main.ts`).
 */
export function registerServices(container: Container): void {
  container.register(INTERPRETER_SERVICE, () => new InterpreterService());
  container.register(EVENT_PARSER_SERVICE, () => new EventParserService());
  container.register(PLAYBACK_SERVICE, (c) => new PlaybackService(c.resolve(EVENT_PARSER_SERVICE)));
}