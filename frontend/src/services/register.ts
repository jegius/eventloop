/**
 * Регистрация сервисов приложения в DI-контейнере.
 *
 * Этот модуль выполняется при импорте (side-effect import) и должен
 * импортироваться ПЕРВЫМ в `main.ts`, ДО импорта компонентов.
 *
 * Это гарантирует, что сервисы зарегистрированы в DI-контейнере до
 * того, как компоненты будут определены и созданы (upgrade custom
 * elements вызывает `connectedCallback`, который резолвит сервисы).
 */

import { container } from '../di/container';
import { registerServices } from './tokens';

registerServices(container);