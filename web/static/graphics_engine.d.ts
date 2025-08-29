/* tslint:disable */
/* eslint-disable */
export class GraphicsEngine {
  free(): void;
  constructor(canvas_id: string);
  set_rotation(rotation: number): void;
  set_scale(scale: number): void;
  set_color(r: number, g: number, b: number): void;
  set_translation(x: number, y: number): void;
  set_background_color(r: number, g: number, b: number, a: number): void;
  set_wireframe_mode(wireframe: boolean): void;
  set_camera_distance(distance: number): void;
  set_camera_angles(angle_x: number, angle_y: number): void;
  render(): void;
  render_cube(): void;
  update_solar_system(delta_time: number): void;
  set_time_scale(scale: number): void;
  render_solar_system(): void;
  get_planet_count(): number;
  get_planet_name(index: number): string;
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_graphicsengine_free: (a: number, b: number) => void;
  readonly graphicsengine_new: (a: number, b: number) => [number, number, number];
  readonly graphicsengine_set_rotation: (a: number, b: number) => void;
  readonly graphicsengine_set_scale: (a: number, b: number) => void;
  readonly graphicsengine_set_color: (a: number, b: number, c: number, d: number) => void;
  readonly graphicsengine_set_translation: (a: number, b: number, c: number) => void;
  readonly graphicsengine_set_background_color: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly graphicsengine_set_wireframe_mode: (a: number, b: number) => void;
  readonly graphicsengine_set_camera_distance: (a: number, b: number) => void;
  readonly graphicsengine_set_camera_angles: (a: number, b: number, c: number) => void;
  readonly graphicsengine_render: (a: number) => void;
  readonly graphicsengine_render_cube: (a: number) => void;
  readonly graphicsengine_update_solar_system: (a: number, b: number) => void;
  readonly graphicsengine_set_time_scale: (a: number, b: number) => void;
  readonly graphicsengine_render_solar_system: (a: number) => void;
  readonly graphicsengine_get_planet_count: (a: number) => number;
  readonly graphicsengine_get_planet_name: (a: number, b: number) => [number, number];
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __externref_table_dealloc: (a: number) => void;
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
