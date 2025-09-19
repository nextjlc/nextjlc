/* src/filetype.ts */

import init, {
  identify_software,
  add_gerber_header,
  process_d_codes,
  add_fingerprint,
  sort_gerber_files,
  map_filenames_ad,
  map_filenames_kicad,
} from "../pkg/nextjlc.js";

export type SoftwareType = "Altium" | "KiCad" | "EasyEDA" | undefined;

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

export async function addGerberHeader(content: string): Promise<string> {
  await initializeWasm();
  return add_gerber_header(content);
}

export async function processDCodes(gerberData: string): Promise<string> {
  await initializeWasm();
  return process_d_codes(gerberData);
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
