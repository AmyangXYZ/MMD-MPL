/* tslint:disable */
/* eslint-disable */
export class MPLBoneState {
  free(): void;
  constructor(bone_name_en: string, bone_name_jp: string, position: Vector3, quaternion: Quaternion);
  readonly bone_name_en: string;
  readonly bone_name_jp: string;
  readonly position: Vector3;
  readonly quaternion: Quaternion;
}
export class Quaternion {
  private constructor();
  free(): void;
  x: number;
  y: number;
  z: number;
  w: number;
}
export class Vector3 {
  private constructor();
  free(): void;
  x: number;
  y: number;
  z: number;
}
export class WasmMPLCompiler {
  free(): void;
  constructor();
  compile(script: string): MPLBoneState[];
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_mplbonestate_free: (a: number, b: number) => void;
  readonly mplbonestate_new: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly mplbonestate_bone_name_en: (a: number, b: number) => void;
  readonly mplbonestate_bone_name_jp: (a: number, b: number) => void;
  readonly mplbonestate_position: (a: number) => number;
  readonly mplbonestate_quaternion: (a: number) => number;
  readonly __wbg_quaternion_free: (a: number, b: number) => void;
  readonly __wbg_get_quaternion_x: (a: number) => number;
  readonly __wbg_set_quaternion_x: (a: number, b: number) => void;
  readonly __wbg_get_quaternion_y: (a: number) => number;
  readonly __wbg_set_quaternion_y: (a: number, b: number) => void;
  readonly __wbg_get_quaternion_z: (a: number) => number;
  readonly __wbg_set_quaternion_z: (a: number, b: number) => void;
  readonly __wbg_get_quaternion_w: (a: number) => number;
  readonly __wbg_set_quaternion_w: (a: number, b: number) => void;
  readonly __wbg_vector3_free: (a: number, b: number) => void;
  readonly __wbg_set_vector3_x: (a: number, b: number) => void;
  readonly __wbg_set_vector3_y: (a: number, b: number) => void;
  readonly __wbg_set_vector3_z: (a: number, b: number) => void;
  readonly __wbg_get_vector3_x: (a: number) => number;
  readonly __wbg_get_vector3_y: (a: number) => number;
  readonly __wbg_get_vector3_z: (a: number) => number;
  readonly __wbg_wasmmplcompiler_free: (a: number, b: number) => void;
  readonly wasmmplcompiler_new: () => number;
  readonly wasmmplcompiler_compile: (a: number, b: number, c: number, d: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
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
