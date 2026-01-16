/* web/filetype.ts */

import init, {
  identify_software,
  get_gerber_header,
  process_d_codes,
  add_fingerprint,
  sort_gerber_files,
  map_filenames_ad,
  map_filenames_kicad,
  get_order_guide_text,
  validate_gerber_files,
  process_drill_files,
  is_drill_file,
  is_through_drill,
  type ValidationResult,
  type DrillProcessResult,
} from "../pkg/nextjlc.js";

export type SoftwareType = "Altium" | "KiCad" | "EasyEDA" | undefined;

// Re-export types for other modules
export type { ValidationResult, DrillProcessResult };

// This promise-based singleton ensures init() is only ever called once.
let initPromise: Promise<void> | null = null;

async function initializeWasm() {
  if (!initPromise) {
    // We just need to run init() once to load and prepare the Wasm module.
    // The high-level named functions will then be available to use.
    initPromise = init().then(() => {
      console.log("Wasm module initialized successfully.");
    });
  }
  return initPromise;
}

// --- Each wrapper now correctly calls the imported high-level function ---

export async function identifyFileType(
  fileContent: string,
): Promise<SoftwareType> {
  await initializeWasm();
  const result = identify_software(fileContent);
  return result as SoftwareType;
}

// MODIFIED: This function is now named getGerberHeader and takes no arguments.
export async function getGerberHeader(): Promise<string> {
  await initializeWasm();
  return get_gerber_header();
}

export async function processDCodes(
  gerberData: string,
  flag: boolean,
): Promise<string> {
  await initializeWasm();
  return process_d_codes(gerberData, flag);
}

export async function addFingerprint(
  content: string,
  isForeign: boolean,
): Promise<string> {
  await initializeWasm();
  return add_fingerprint(content, isForeign);
}

export async function sortGerberFiles(files: string[]): Promise<string[]> {
  await initializeWasm();
  return sort_gerber_files(files);
}

export async function mapFilenames(
  files: string[],
  edaType: "Altium" | "KiCad",
): Promise<Map<string, string>> {
  await initializeWasm();
  if (edaType === "Altium") {
    return map_filenames_ad(files);
  } else {
    return map_filenames_kicad(files);
  }
}

// Wrapper function for getting the order guide text.
export async function getOrderGuideText(): Promise<string> {
  await initializeWasm();
  return get_order_guide_text();
}

// Wrapper for the file validation function.
export async function validateGerberFiles(
  files: string[],
): Promise<ValidationResult> {
  await initializeWasm();
  return validate_gerber_files(files);
}

// Wrapper to check if a file is a drill file.
export async function isDrillFile(filename: string): Promise<boolean> {
  await initializeWasm();
  return is_drill_file(filename);
}

// Wrapper to check if a drill file is a through-hole file (not blind/buried).
export async function isThroughDrill(filename: string): Promise<boolean> {
  await initializeWasm();
  return is_through_drill(filename);
}

// Wrapper to process drill files and generate PTH/NPTH content.
export async function processDrillFiles(
  contents: string[],
  filenames: string[],
): Promise<DrillProcessResult> {
  await initializeWasm();
  return process_drill_files(contents, filenames);
}
