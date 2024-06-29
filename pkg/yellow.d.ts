/* tslint:disable */
/* eslint-disable */
/**
* @param {string} ev
*/
export function keyboard_event_up(ev: string): void;
/**
* @param {string} ev
*/
export function keyboard_event_down(ev: string): void;
/**
*/
export function update_event(): void;
/**
* @param {number} width
* @param {number} height
*/
export function resize_event(width: number, height: number): void;
/**
* @param {string} canvas
*/
export function main(canvas: string): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly keyboard_event_up: (a: number, b: number) => void;
  readonly keyboard_event_down: (a: number, b: number) => void;
  readonly update_event: () => void;
  readonly resize_event: (a: number, b: number) => void;
  readonly main: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
