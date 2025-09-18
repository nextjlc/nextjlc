/* src/editfile.ts */

import { create } from "zustand";

// Define the shape of our state
interface WorkflowState {
  workflowState: "upload" | "process";
  files: string[];
}

// Define the actions that can be performed on the state
interface WorkflowActions {
  setProcessState: (files: string[]) => void;
  resetWorkflow: () => void;
}

// Create the store
export const useWorkflowStore = create<WorkflowState & WorkflowActions>(
  (set) => ({
    // Initial state
    workflowState: "upload",
    files: [],

    // Action to move to the 'process' state with a list of files
    setProcessState: (files) => set({ workflowState: "process", files: files }),

    // Action to reset everything back to the 'upload' state
    resetWorkflow: () => set({ workflowState: "upload", files: [] }),
  }),
);
