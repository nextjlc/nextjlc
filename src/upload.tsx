/* src/upload.tsx */

import { useState, useRef } from "react";
import type { DragEvent } from "react";
import { UploadCloud } from "lucide-react";

function UploadZone() {
  const [isDragging, setIsDragging] = useState(false);
  const fileInputRef = useRef<HTMLInputElement>(null);

  const handleFile = (file: File) => {
    if (file && file.name.endsWith(".zip")) {
      console.log("Accepted file:", file.name);
    } else {
      alert("Invalid file type. Please upload a .zip file.");
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
    const files = e.dataTransfer.files;
    if (files && files.length > 0) {
      handleFile(files[0]);
    }
  };

  const handleDivClick = () => {
    fileInputRef.current?.click();
  };
  const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    const files = e.target.files;
    if (files && files.length > 0) {
      handleFile(files[0]);
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
        className={`
          h-16 w-16 mb-4
          ${isDragging ? "text-[var(--color-accent)]" : "text-gray-500"}
        `}
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
