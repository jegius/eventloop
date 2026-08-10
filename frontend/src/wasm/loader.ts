/**
 * Загрузчик WASM-модуля интерпретатора.
 *
 * Загружает скомпилированный Rust-интерпретатор (js_eventloop_interpreter)
 * и предоставляет API для выполнения JS-кода.
 *
 * WASM-модуль компилируется из `interpreter/` через `wasm-pack build --target web`.
 */

import type { RunResult } from '../types';
import init, { run as wasmRun, step as wasmStep } from './js_eventloop_interpreter';

/** Интерфейс WASM-модуля. */
export interface InterpreterWasm {
  run(source: string): RunResult;
  step(source: string, stepIndex: number): RunResult;
}

let wasmInstance: InterpreterWasm | null = null;

/**
 * Загружает WASM-модуль интерпретатора.
 */
export async function loadInterpreter(): Promise<InterpreterWasm> {
  if (wasmInstance) {
    return wasmInstance;
  }

  // Инициализация WASM-модуля
  await init();

  wasmInstance = {
    run(source: string): RunResult {
      const result = wasmRun(source);
      return {
        events: result.events,
        consoleOutput: result.console_output,
      };
    },
    step(source: string, stepIndex: number): RunResult {
      const result = wasmStep(source, stepIndex);
      return {
        events: result.events,
        consoleOutput: result.console_output,
      };
    },
  };

  return wasmInstance;
}

/** Выполняет JS-код через интерпретатор. */
export async function runCode(source: string): Promise<RunResult> {
  const interpreter = await loadInterpreter();
  return interpreter.run(source);
}

/** Выполняет один шаг (для пошагового режима). */
export async function stepCode(source: string, stepIndex: number): Promise<RunResult> {
  const interpreter = await loadInterpreter();
  return interpreter.step(source, stepIndex);
}