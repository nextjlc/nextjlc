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
  // Action to copy or update a single file in the processed list.
  copyFileToProcessed: (file: ProcessedGerberFile) => void;
  // Action to remove a single file from the processed list.
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

    // Implementation for copying/updating a file.
    // It checks if a file with the same *new* name already exists.
    // If it does, it updates it (overwrite). If not, it adds it to the list.
    copyFileToProcessed: (fileToCopy) =>
      set((state) => {
        const existingIndex = state.processedFiles.findIndex(
          (f) => f.newName === fileToCopy.newName,
        );
        let updatedFiles;
        if (existingIndex > -1) {
          // File with same newName exists, replace it.
          updatedFiles = [...state.processedFiles];
          updatedFiles[existingIndex] = fileToCopy;
        } else {
          // File does not exist, add it.
          updatedFiles = [...state.processedFiles, fileToCopy];
        }
        return { processedFiles: updatedFiles };
      }),

    // Implementation for removing a file.
    // It filters the list, keeping all files except the one matching the originalName.
    removeFileFromProcessed: (originalNameToRemove) =>
      set((state) => ({
        processedFiles: state.processedFiles.filter(
          (f) => f.originalName !== originalNameToRemove,
        ),
      })),
  }),
);
