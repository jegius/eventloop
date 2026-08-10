/* tslint:disable */
/* eslint-disable */

/**
 * Результат выполнения кода: трасса событий Event Loop и вывод консоли.
 */
export class RunResult {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * Вывод консоли.
     */
    readonly console_output: string[];
    /**
     * События Event Loop в JSON-формате.
     */
    readonly events: string[];
}

/**
 * Выполняет JS-код и возвращает трассу событий Event Loop.
 *
 * # Аргументы
 * - `source` — исходный код на JavaScript.
 *
 * # Возвращает
 * Результат выполнения: события Event Loop и вывод консоли.
 */
export function run(source: string): RunResult;

/**
 * Выполнить один шаг Event Loop (для пошагового режима визуализации).
 *
 * # Аргументы
 * - `source`: исходный код.
 * - `step_index`: индекс шага, с которого продолжить.
 */
export function step(source: string, step_index: number): RunResult;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly __wbg_runresult_free: (a: number, b: number) => void;
    readonly run: (a: number, b: number) => number;
    readonly runresult_console_output: (a: number) => [number, number];
    readonly runresult_events: (a: number) => [number, number];
    readonly step: (a: number, b: number, c: number) => number;
    readonly __wbindgen_externrefs: WebAssembly.Table;
    readonly __wbindgen_malloc: (a: number, b: number) => number;
    readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
    readonly __externref_drop_slice: (a: number, b: number) => void;
    readonly __wbindgen_free: (a: number, b: number, c: number) => void;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
