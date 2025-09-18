/* src/filetype.ts */

import init, { identify_software } from "../pkg/nextjlc.js";

export type SoftwareType = "Altium" | "KiCad" | "EasyEDA" | undefined;

let initPromise: Promise<void> | null = null;

async function initializeWasm() {
  if (!initPromise) {
    initPromise = init().then(() => {
      console.log("Wasm module initialized successfully.");
    });
  }
  return initPromise;
}

/**
 * Analyzes the content of a file to identify the EDA software.
 * @param fileContent The string content of the Gerber file.
 * @returns A promise that resolves to the identified software name or undefined.
 */
export async function identifyFileType(
  fileContent: string,
): Promise<SoftwareType> {
  await initializeWasm();
  const result = identify_software(fileContent);
  return result as SoftwareType;
}
