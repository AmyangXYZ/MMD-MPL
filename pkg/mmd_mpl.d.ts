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
  free(): void;
  constructor(x: number, y: number, z: number, w: number);
  static identity(): Quaternion;
  multiply(other: Quaternion): Quaternion;
  dot(other: Quaternion): number;
  similarity(other: Quaternion): number;
  angular_distance(other: Quaternion): number;
  static from_axis_angle(axis: Vector3, degrees: number): Quaternion;
  x: number;
  y: number;
  z: number;
  w: number;
}
export class Vector3 {
  free(): void;
  constructor(x: number, y: number, z: number);
  normalize(): Vector3;
  /**
   * Dot product with another vector
   */
  dot(other: Vector3): number;
  x: number;
  y: number;
  z: number;
}
export class WasmMPLCompiler {
  free(): void;
  constructor();
  compile(script: string): MPLBoneState[];
  reverse_compile(name: string, states: MPLBoneState[]): string;
  get_all_bones(): string[];
  get_bone_actions(bone: string): string[] | undefined;
  get_bone_directions(bone: string, action: string): string[] | undefined;
  get_bone_degree_limit(bone: string, action: string, direction: string): number | undefined;
  get_bone_japanese_name(bone: string): string | undefined;
  get_bone_english_name(bone: string): string | undefined;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_mplbonestate_free: (a: number, b: number) => void;
  readonly mplbonestate_new: (a: number, b: number, c: number, d: number, e: number, f: number) => number;
  readonly mplbonestate_bone_name_en: (a: number) => [number, number];
  readonly mplbonestate_bone_name_jp: (a: number) => [number, number];
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
  readonly quaternion_new: (a: number, b: number, c: number, d: number) => number;
  readonly quaternion_identity: () => number;
  readonly quaternion_multiply: (a: number, b: number) => number;
  readonly quaternion_dot: (a: number, b: number) => number;
  readonly quaternion_similarity: (a: number, b: number) => number;
  readonly quaternion_angular_distance: (a: number, b: number) => number;
  readonly quaternion_from_axis_angle: (a: number, b: number) => number;
  readonly __wbg_vector3_free: (a: number, b: number) => void;
  readonly vector3_new: (a: number, b: number, c: number) => number;
  readonly vector3_normalize: (a: number) => number;
  readonly vector3_dot: (a: number, b: number) => number;
  readonly __wbg_wasmmplcompiler_free: (a: number, b: number) => void;
  readonly wasmmplcompiler_new: () => number;
  readonly wasmmplcompiler_compile: (a: number, b: number, c: number) => [number, number, number, number];
  readonly wasmmplcompiler_reverse_compile: (a: number, b: number, c: number, d: number, e: number) => [number, number];
  readonly wasmmplcompiler_get_all_bones: (a: number) => [number, number];
  readonly wasmmplcompiler_get_bone_actions: (a: number, b: number, c: number) => [number, number];
  readonly wasmmplcompiler_get_bone_directions: (a: number, b: number, c: number, d: number, e: number) => [number, number];
  readonly wasmmplcompiler_get_bone_degree_limit: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => number;
  readonly wasmmplcompiler_get_bone_japanese_name: (a: number, b: number, c: number) => [number, number];
  readonly wasmmplcompiler_get_bone_english_name: (a: number, b: number, c: number) => [number, number];
  readonly __wbg_get_vector3_x: (a: number) => number;
  readonly __wbg_get_vector3_y: (a: number) => number;
  readonly __wbg_get_vector3_z: (a: number) => number;
  readonly __wbg_set_vector3_x: (a: number, b: number) => void;
  readonly __wbg_set_vector3_y: (a: number, b: number) => void;
  readonly __wbg_set_vector3_z: (a: number, b: number) => void;
  readonly __wbindgen_export_0: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __externref_drop_slice: (a: number, b: number) => void;
  readonly __externref_table_alloc: () => number;
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
