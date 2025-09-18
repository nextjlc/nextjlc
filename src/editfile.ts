/* src/editfile.ts */

import { create } from "zustand";
import type JSZip from "jszip";

export type SoftwareType = "Altium" | "KiCad" | "EasyEDA" | "None";

export interface GerberFile {
  name: string;
  fileObject: JSZip.JSZipObject;
  software?: SoftwareType; // The type is now more specific
}

interface WorkflowState {
  workflowState: "upload" | "process";
  files: GerberFile[];
}

interface WorkflowActions {
  setProcessState: (files: GerberFile[]) => void;
  setFileSoftware: (fileName: string, software: SoftwareType) => void;
  resetWorkflow: () => void;
}

export const useWorkflowStore = create<WorkflowState & WorkflowActions>(
  (set) => ({
    workflowState: "upload",
    files: [],
    setProcessState: (files) => set({ workflowState: "process", files: files }),
    resetWorkflow: () => set({ workflowState: "upload", files: [] }),
    setFileSoftware: (fileName, software) =>
      set((state) => ({
        files: state.files.map((f) =>
          f.name === fileName ? { ...f, software: software } : f,
        ),
      })),
  }),
);
