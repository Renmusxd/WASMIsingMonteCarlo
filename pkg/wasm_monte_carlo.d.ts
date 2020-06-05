/* tslint:disable */
/* eslint-disable */
/**
*/
export function init_panic_hook(): void;
/**
*/
export class Lattice {
  free(): void;
/**
* Make a new lattice with `edges`, positive implies antiferromagnetic bonds, negative is
* ferromagnetic.
* @param {Uint32Array} edge_a 
* @param {Uint32Array} edge_b 
* @param {Float64Array} edge_j 
* @param {number} transverse 
* @param {number} beta 
* @returns {Lattice} 
*/
  static new(edge_a: Uint32Array, edge_b: Uint32Array, edge_j: Float64Array, transverse: number, beta: number): Lattice;
/**
* Run a quantum monte carlo simulation, return the average energy.
*
* # Arguments:
* * `timesteps`: number of timesteps to run.
*
* # Returns:
*  * average energy of system
* @param {number} timesteps 
* @returns {number} 
*/
  run_quantum_monte_carlo(timesteps: number): number;
/**
* @returns {number} 
*/
  get_nvars(): number;
/**
* @param {number} i 
* @returns {boolean} 
*/
  get_state(i: number): boolean;
/**
* @returns {number} 
*/
  get_n(): number;
/**
* @returns {number} 
*/
  get_cutoff(): number;
/**
* @param {number} n 
* @returns {number} 
*/
  get_p_for_n(n: number): number;
/**
* @param {number} p 
* @returns {number | undefined} 
*/
  get_nvars_for_op(p: number): number | undefined;
/**
* @param {number} p 
* @param {number} i 
* @returns {number | undefined} 
*/
  get_nth_op_var_i(p: number, i: number): number | undefined;
/**
* @param {number} p 
* @param {number} i 
* @returns {boolean | undefined} 
*/
  get_nth_op_var_i_input(p: number, i: number): boolean | undefined;
/**
* @param {number} p 
* @param {number} i 
* @returns {boolean | undefined} 
*/
  get_nth_op_var_i_output(p: number, i: number): boolean | undefined;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_lattice_free: (a: number) => void;
  readonly lattice_new: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number) => number;
  readonly lattice_run_quantum_monte_carlo: (a: number, b: number) => number;
  readonly lattice_get_nvars: (a: number) => number;
  readonly lattice_get_state: (a: number, b: number) => number;
  readonly lattice_get_n: (a: number) => number;
  readonly lattice_get_cutoff: (a: number) => number;
  readonly lattice_get_p_for_n: (a: number, b: number) => number;
  readonly lattice_get_nvars_for_op: (a: number, b: number, c: number) => void;
  readonly lattice_get_nth_op_var_i: (a: number, b: number, c: number, d: number) => void;
  readonly lattice_get_nth_op_var_i_input: (a: number, b: number, c: number) => number;
  readonly lattice_get_nth_op_var_i_output: (a: number, b: number, c: number) => number;
  readonly init_panic_hook: () => void;
  readonly __wbindgen_malloc: (a: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __wbindgen_free: (a: number, b: number) => void;
  readonly __wbindgen_realloc: (a: number, b: number, c: number) => number;
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
        