/**
 * Сервис интерпретатора.
 *
 * Предоставляет доступ к WASM-модулю Rust-интерпретатора через
 * единый API. Инкапсулирует загрузку WASM и выполнение JS-кода.
 *
 * Внедряется через DI-контейнер, что позволяет подменять реализацию
 * (например, моком) в тестах.
 */

import { runCode, stepCode } from '../wasm/loader';
import type { RunResult } from '../types';

/**
 * Сервис интерпретатора.
 *
 * Реализует паттерн Service: инкапсулирует бизнес-логику взаимодействия
 * с WASM-интерпретатором и предоставляет её компонентам через DI.
 */
export class InterpreterService {
  /**
   * Выполняет JS-код через интерпретатор и возвращает результат.
   *
   * @param source Исходный JS-код.
   * @returns Результат выполнения (события Event Loop и вывод консоли).
   */
  async run(source: string): Promise<RunResult> {
    return runCode(source);
  }

  /**
   * Выполняет один шаг интерпретации (для пошагового режима).
   *
   * @param source    Исходный JS-код.
   * @param stepIndex Индекс шага.
   * @returns Результат выполнения одного шага.
   */
  async step(source: string, stepIndex: number): Promise<RunResult> {
    return stepCode(source, stepIndex);
  }
}