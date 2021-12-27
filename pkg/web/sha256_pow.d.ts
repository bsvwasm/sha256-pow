/* tslint:disable */
/* eslint-disable */
/**
* Takes a template vector to be hashed and a target difficulty u256 in hex string form and will mine until it finds a hash that matches the desired difficulty
* @param {Uint8Array} template
* @param {string} target_hex
* @returns {Promise<ProofOfWork>}
*/
export function mine(template: Uint8Array, target_hex: string): Promise<ProofOfWork>;
/**
* @param {ProofOfWork} pow
* @param {string} target_hex
* @returns {boolean}
*/
export function verify(pow: ProofOfWork, target_hex: string): boolean;
/**
*/
export enum POWErrors {
  InvalidTarget,
  CantFindTarget,
}
/**
*/
export class ProofOfWork {
  free(): void;
/**
* @returns {Uint8Array}
*/
  get_hash(): Uint8Array;
/**
* @returns {Uint8Array}
*/
  get_template(): Uint8Array;
/**
* @returns {BigInt}
*/
  get_nonce(): BigInt;
/**
* @returns {string}
*/
  to_json(): string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_proofofwork_free: (a: number) => void;
  readonly proofofwork_get_hash: (a: number, b: number) => void;
  readonly proofofwork_get_template: (a: number, b: number) => void;
  readonly proofofwork_get_nonce: (a: number, b: number) => void;
  readonly proofofwork_to_json: (a: number, b: number) => void;
  readonly mine: (a: number, b: number, c: number, d: number) => number;
  readonly verify: (a: number, b: number, c: number) => number;
  readonly __wbindgen_export_0: WebAssembly.Table;
  readonly __wbindgen_export_1: WebAssembly.Table;
  readonly closure26_externref_shim: (a: number, b: number, c: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly closure45_externref_shim: (a: number, b: number, c: number, d: number) => void;
  readonly __wbindgen_start: () => void;
}

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
