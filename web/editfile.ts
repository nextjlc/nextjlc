/* web/editfile.ts */

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
  layerCount: number | null; // ADDED: To store the detected layer count.
}

interface WorkflowActions {
  setProcessState: (files: GerberFile[], originalName: string) => void;
  setFileSoftware: (fileName: string, software: SoftwareType) => void;
  startProcessing: () => void;
  setProcessedFiles: (
    files: ProcessedGerberFile[],
    primaryEda: SoftwareType | null,
    layerCount: number | null, // MODIFIED: Accept layer count.
  ) => void;
  setProgress: (progress: number) => void;
  resetWorkflow: () => void;
  copyFileToProcessed: (file: ProcessedGerberFile) => void;
  removeFileFromProcessed: (originalName: string) => void;
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
    layerCount: null, // ADDED: Initial state.
    setProcessState: (files, originalName) =>
      set({
        workflowState: "process",
        files: files,
        originalZipName: originalName,
        processedFiles: [],
        primaryEda: null,
        progress: 0,
        layerCount: null, // Ensure layerCount is reset on new file upload.
      }),
    resetWorkflow: () =>
      set({
        workflowState: "upload",
        files: [],
        processedFiles: [],
        originalZipName: null,
        primaryEda: null,
        progress: 0,
        layerCount: null, // Ensure layerCount is reset.
      }),
    setFileSoftware: (fileName, software) =>
      set((state) => ({
        files: state.files.map((f) =>
          f.name === fileName ? { ...f, software: software } : f,
        ),
      })),
    startProcessing: () =>
      set({
        isProcessing: true,
        processedFiles: [],
        progress: 0,
        layerCount: null,
      }),
    // MODIFIED: This action now also sets the layer count.
    setProcessedFiles: (files, primaryEda, layerCount) =>
      set({
        processedFiles: files,
        isProcessing: false,
        primaryEda: primaryEda,
        layerCount: layerCount, // Set the layer count in the state.
      }),
    setProgress: (progress) => set({ progress }),

    copyFileToProcessed: (fileToCopy) =>
      set((state) => {
        const existingIndex = state.processedFiles.findIndex(
          (f) => f.newName === fileToCopy.newName,
        );
        let updatedFiles;
        if (existingIndex > -1) {
          updatedFiles = [...state.processedFiles];
          updatedFiles[existingIndex] = fileToCopy;
        } else {
          updatedFiles = [...state.processedFiles, fileToCopy];
        }
        return { processedFiles: updatedFiles };
      }),

    removeFileFromProcessed: (originalNameToRemove) =>
      set((state) => ({
        processedFiles: state.processedFiles.filter(
          (f) => f.originalName !== originalNameToRemove,
        ),
      })),
  }),
);
