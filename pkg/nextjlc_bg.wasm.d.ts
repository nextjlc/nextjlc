/* pkg/nextjlc_bg.wasm.d.ts */

/* eslint-disable */
export const memory: WebAssembly.Memory;
export const process_d_codes: (
  a: number,
  b: number,
  c: number,
) => [number, number];
export const identify_software: (a: number, b: number) => [number, number];
export const add_fingerprint: (
  a: number,
  b: number,
  c: number,
) => [number, number];
export const get_gerber_header: () => [number, number];
export const get_order_guide_text: () => [number, number];
export const sort_gerber_files: (a: number, b: number) => [number, number];
export const map_filenames_ad: (a: number, b: number) => any;
export const map_filenames_kicad: (a: number, b: number) => any;
export const __wbg_validationresult_free: (a: number, b: number) => void;
export const __wbg_get_validationresult_is_valid: (a: number) => number;
export const __wbg_set_validationresult_is_valid: (
  a: number,
  b: number,
) => void;
export const __wbg_get_validationresult_layer_count: (a: number) => number;
export const __wbg_set_validationresult_layer_count: (
  a: number,
  b: number,
) => void;
export const validationresult_warnings: (a: number) => [number, number];
export const validationresult_errors: (a: number) => [number, number];
export const validate_gerber_files: (a: number, b: number) => number;
export const __wbindgen_exn_store: (a: number) => void;
export const __externref_table_alloc: () => number;
export const __wbindgen_export_2: WebAssembly.Table;
export const __wbindgen_malloc: (a: number, b: number) => number;
export const __wbindgen_realloc: (
  a: number,
  b: number,
  c: number,
  d: number,
) => number;
export const __wbindgen_free: (a: number, b: number, c: number) => void;
export const __externref_drop_slice: (a: number, b: number) => void;
export const __wbindgen_start: () => void;
