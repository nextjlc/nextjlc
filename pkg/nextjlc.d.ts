/* tslint:disable */
/* eslint-disable */
export function process_d_codes(
  gerber_data: string,
  use_altium: boolean,
): string;
export function identify_software(content: string): string | undefined;
export function add_fingerprint(
  gerber_content: string,
  is_foreign_board_file: boolean,
): string;
export function get_gerber_header(): string;
export function get_order_guide_text(): string;
export function sort_gerber_files(files: string[]): string[];
export function map_filenames_ad(files: string[]): Map<any, any>;
export function map_filenames_kicad(files: string[]): Map<any, any>;
export function validate_gerber_files(files: string[]): ValidationResult;
export class ValidationResult {
  private constructor();
  free(): void;
  is_valid: boolean;
  layer_count: number;
  readonly warnings: string[];
  readonly errors: string[];
}

export type InitInput =
  | RequestInfo
  | URL
  | Response
  | BufferSource
  | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly process_d_codes: (
    a: number,
    b: number,
    c: number,
  ) => [number, number];
  readonly identify_software: (a: number, b: number) => [number, number];
  readonly add_fingerprint: (
    a: number,
    b: number,
    c: number,
  ) => [number, number];
  readonly get_gerber_header: () => [number, number];
  readonly get_order_guide_text: () => [number, number];
  readonly sort_gerber_files: (a: number, b: number) => [number, number];
  readonly map_filenames_ad: (a: number, b: number) => any;
  readonly map_filenames_kicad: (a: number, b: number) => any;
  readonly __wbg_validationresult_free: (a: number, b: number) => void;
  readonly __wbg_get_validationresult_is_valid: (a: number) => number;
  readonly __wbg_set_validationresult_is_valid: (a: number, b: number) => void;
  readonly __wbg_get_validationresult_layer_count: (a: number) => number;
  readonly __wbg_set_validationresult_layer_count: (
    a: number,
    b: number,
  ) => void;
  readonly validationresult_warnings: (a: number) => [number, number];
  readonly validationresult_errors: (a: number) => [number, number];
  readonly validate_gerber_files: (a: number, b: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_export_2: WebAssembly.Table;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (
    a: number,
    b: number,
    c: number,
    d: number,
  ) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __externref_drop_slice: (a: number, b: number) => void;
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
export function initSync(
  module: { module: SyncInitInput } | SyncInitInput,
): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init(
  module_or_path?:
    | { module_or_path: InitInput | Promise<InitInput> }
    | InitInput
    | Promise<InitInput>,
): Promise<InitOutput>;
