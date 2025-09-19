/* src/upload.tsx */

import { useState, useRef } from "react";
import type { DragEvent } from "react";
import { UploadCloud } from "lucide-react";
import JSZip from "jszip";
import { useWorkflowStore } from "./editfile";
import type { GerberFile } from "./editfile";

function UploadZone() {
  const [isDragging, setIsDragging] = useState(false);
  const fileInputRef = useRef<HTMLInputElement>(null);
  const { setProcessState, resetWorkflow } = useWorkflowStore();

  const handleFile = async (file: File) => {
    if (!file || !file.name.endsWith(".zip")) {
      alert("Invalid file type. Please upload a .zip file.");
      return;
    }

    try {
      const zip = await JSZip.loadAsync(file);
      const zipEntries = Object.values(zip.files);
      let filesToProcess: JSZip.JSZipObject[] = [];

      const rootFiles = zipEntries.filter(
        (entry) => !entry.dir && entry.name.indexOf("/") === -1,
      );
      if (rootFiles.length >= 2) {
        filesToProcess = rootFiles;
      } else if (rootFiles.length === 0) {
        const rootFolders = zipEntries.filter(
          (entry) => entry.dir && entry.name.split("/").length === 2,
        );
        if (rootFolders.length === 1) {
          const singleFolderName = rootFolders[0].name;
          filesToProcess = zipEntries.filter(
            (entry) =>
              !entry.dir &&
              entry.name.startsWith(singleFolderName) &&
              entry.name.substring(singleFolderName.length).indexOf("/") === -1,
          );
        }
      }

      if (filesToProcess.length >= 2) {
        const gerberFiles: GerberFile[] = filesToProcess.map((entry) => ({
          name: entry.name.split("/").pop() || entry.name,
          fileObject: entry,
        }));
        setProcessState(gerberFiles, file.name);
      } else {
        alert(
          "Invalid zip structure. Please ensure there are at least two Gerber files.",
        );
        resetWorkflow();
      }
    } catch (error) {
      console.error("Error processing zip file:", error);
      alert("Could not read the zip file. It may be corrupt.");
      resetWorkflow();
    }
  };

  const handleDragOver = (e: DragEvent<HTMLDivElement>) => {
    e.preventDefault();
  };
  const handleDragEnter = (e: DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    setIsDragging(true);
  };
  const handleDragLeave = () => {
    setIsDragging(false);
  };
  const handleDrop = (e: DragEvent<HTMLDivElement>) => {
    e.preventDefault();
    setIsDragging(false);
    if (e.dataTransfer.files && e.dataTransfer.files.length > 0) {
      handleFile(e.dataTransfer.files[0]);
    }
  };
  const handleDivClick = () => {
    fileInputRef.current?.click();
  };
  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    if (e.target.files && e.target.files.length > 0) {
      handleFile(e.target.files[0]);
    }
  };

  return (
    <div
      onClick={handleDivClick}
      onDragOver={handleDragOver}
      onDragEnter={handleDragEnter}
      onDragLeave={handleDragLeave}
      onDrop={handleDrop}
      className={`
        w-full max-w-lg p-10 border-2 border-dashed rounded-lg cursor-pointer
        flex flex-col items-center justify-center text-center
        transition-colors duration-200
        ${
          isDragging
            ? "border-[var(--color-accent)] bg-[color:var(--color-accent)/0.1]"
            : "border-gray-500 hover:border-[var(--color-accent)] hover:bg-[color:var(--color-accent)/0.1]"
        }
      `}
    >
      <input
        type="file"
        ref={fileInputRef}
        onChange={handleFileChange}
        className="hidden"
        accept=".zip"
      />
      <UploadCloud
        className={`h-16 w-16 mb-4 ${isDragging ? "text-[var(--color-accent)]" : "text-gray-500"}`}
      />
      <p className="text-lg font-semibold text-[var(--color-text)]">
        Click to browse or drag & drop a file
      </p>
      <p className="text-sm text-[var(--color-subtext)] mt-1">
        Only .zip Gerber files are accepted
      </p>
    </div>
  );
}

export default UploadZone;
