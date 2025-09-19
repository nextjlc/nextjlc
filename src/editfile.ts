/* src/editfile.ts */

import { create } from "zustand";
import type JSZip from "jszip";

export type SoftwareType = "Altium" | "KiCad" | "EasyEDA" | "None";

export interface GerberFile {
  name: string;
  fileObject: JSZip.JSZipObject;
  software?: SoftwareType;
}

export interface ProcessedGerberFile {
  originalName: string;
  newName: string;
  content: string;
}

interface WorkflowState {
  workflowState: "upload" | "process";
  originalZipName: string | null; // To store the original filename
  primaryEda: SoftwareType | null; // To store the detected primary EDA
  files: GerberFile[];
  processedFiles: ProcessedGerberFile[];
  isProcessing: boolean;
}

interface WorkflowActions {
  setProcessState: (files: GerberFile[], originalName: string) => void; // Pass in the name
  setFileSoftware: (fileName: string, software: SoftwareType) => void;
  startProcessing: () => void;
  setProcessedFiles: (
    files: ProcessedGerberFile[],
    primaryEda: SoftwareType,
  ) => void; // Pass in the EDA
  resetWorkflow: () => void;
}

export const useWorkflowStore = create<WorkflowState & WorkflowActions>(
  (set) => ({
    workflowState: "upload",
    files: [],
    processedFiles: [],
    isProcessing: false,
    originalZipName: null,
    primaryEda: null,
    setProcessState: (files, originalName) =>
      set({
        workflowState: "process",
        files: files,
        originalZipName: originalName,
        processedFiles: [], // Clear previous results on new upload
        primaryEda: null,
      }),
    resetWorkflow: () =>
      set({
        workflowState: "upload",
        files: [],
        processedFiles: [],
        originalZipName: null,
        primaryEda: null,
      }),
    setFileSoftware: (fileName, software) =>
      set((state) => ({
        files: state.files.map((f) =>
          f.name === fileName ? { ...f, software: software } : f,
        ),
      })),
    startProcessing: () => set({ isProcessing: true, processedFiles: [] }),
    setProcessedFiles: (files, primaryEda) =>
      set({
        processedFiles: files,
        isProcessing: false,
        primaryEda: primaryEda,
      }),
  }),
);
