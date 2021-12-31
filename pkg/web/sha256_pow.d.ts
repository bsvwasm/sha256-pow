/* tslint:disable */
/* eslint-disable */
/**
* Takes a template vector to be hashed and a target difficulty u256 in hex string form and will mine until it finds a hash that matches the desired difficulty
* @param {Uint8Array} template
* @param {string} target_hex
* @param {BigInt | undefined} offset
* @returns {Promise<ProofOfWork>}
*/
export function mine(template: Uint8Array, target_hex: string, offset?: BigInt): Promise<ProofOfWork>;
/**
* Takes a template vector to be hashed and a target difficulty u256 in hex string form and will mine until it finds a hash that matches the desired difficulty
* @param {Uint8Array} template
* @param {string} target_hex
* @returns {Promise<ProofOfWork>}
*/
export function mine_random(template: Uint8Array, target_hex: string): Promise<ProofOfWork>;
/**
* @param {ProofOfWork} pow
* @param {string} target_hex
* @returns {boolean}
*/
export function verify(pow: ProofOfWork, target_hex: string): boolean;
/**
* @param {Uint8Array} a
* @param {Uint8Array} b
* @returns {Uint8Array}
*/
export function xor(a: Uint8Array, b: Uint8Array): Uint8Array;
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
* @returns {string}
*/
  get_hash_hex(): string;
/**
* @returns {string}
*/
  get_template_hex(): string;
/**
* @returns {BigInt}
*/
  get_nonce(): BigInt;
/**
* @returns {any}
*/
  to_json(): any;
/**
* @param {any} json_obj
* @returns {ProofOfWork}
*/
  static from_json(json_obj: any): ProofOfWork;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_proofofwork_free: (a: number) => void;
  readonly proofofwork_get_hash: (a: number, b: number) => void;
  readonly proofofwork_get_template: (a: number, b: number) => void;
  readonly proofofwork_get_hash_hex: (a: number, b: number) => void;
  readonly proofofwork_get_template_hex: (a: number, b: number) => void;
  readonly proofofwork_get_nonce: (a: number, b: number) => void;
  readonly proofofwork_to_json: (a: number) => number;
  readonly proofofwork_from_json: (a: number) => number;
  readonly mine: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => number;
  readonly mine_random: (a: number, b: number, c: number, d: number) => number;
  readonly verify: (a: number, b: number, c: number) => number;
  readonly xor: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_export_3: WebAssembly.Table;
  readonly closure53_externref_shim: (a: number, b: number, c: number) => void;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly closure92_externref_shim: (a: number, b: number, c: number, d: number) => void;
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
