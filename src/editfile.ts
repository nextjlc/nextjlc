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
  originalZipName: string | null;
  primaryEda: SoftwareType | null;
  files: GerberFile[];
  processedFiles: ProcessedGerberFile[];
  isProcessing: boolean;
  progress: number;
}

interface WorkflowActions {
  setProcessState: (files: GerberFile[], originalName: string) => void;
  setFileSoftware: (fileName: string, software: SoftwareType) => void;
  startProcessing: () => void;
  setProcessedFiles: (
    files: ProcessedGerberFile[],
    primaryEda: SoftwareType,
  ) => void;
  setProgress: (progress: number) => void;
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
    progress: 0,
    setProcessState: (files, originalName) =>
      set({
        workflowState: "process",
        files: files,
        originalZipName: originalName,
        processedFiles: [],
        primaryEda: null,
        progress: 0,
      }),
    resetWorkflow: () =>
      set({
        workflowState: "upload",
        files: [],
        processedFiles: [],
        originalZipName: null,
        primaryEda: null,
        progress: 0,
      }),
    setFileSoftware: (fileName, software) =>
      set((state) => ({
        files: state.files.map((f) =>
          f.name === fileName ? { ...f, software: software } : f,
        ),
      })),
    startProcessing: () =>
      set({ isProcessing: true, processedFiles: [], progress: 0 }),
    setProcessedFiles: (files, primaryEda) =>
      set({
        processedFiles: files,
        isProcessing: false,
        primaryEda: primaryEda,
      }),
    setProgress: (progress) => set({ progress }),
  }),
);
