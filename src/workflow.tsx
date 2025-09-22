/* src/workflow.tsx */

import { useEffect, useMemo, useState } from "react";
import { useWorkflowStore } from "./editfile";
import {
  PackageSearch,
  Archive,
  LoaderCircle,
  ArrowRight,
  X,
} from "lucide-react";
import JSZip from "jszip";
import {
  identifyFileType,
  addGerberHeader,
  processDCodes,
  addFingerprint,
  sortGerberFiles,
  mapFilenames,
  getOrderGuideText,
} from "./filetype";
import { HeaderBadge } from "./header-badge";
import type { GerberFile, ProcessedGerberFile } from "./editfile";

function Workflow() {
  const {
    files,
    setFileSoftware,
    processedFiles,
    isProcessing,
    startProcessing,
    setProcessedFiles,
    originalZipName,
    primaryEda,
    setProgress,
    copyFileToProcessed,
    removeFileFromProcessed,
  } = useWorkflowStore();

  const [isAnalysisComplete, setIsAnalysisComplete] = useState(false);
  // State to hold the rename map after processing, so individual copy actions can use it.
  const [renameMap, setRenameMap] = useState<Map<string, string>>(new Map());

  // --- Phase 1: Analysis Progress ---
  useEffect(() => {
    // This effect remains unchanged.
    const allFilesAnalyzed = files.every((f) => f.software !== undefined);
    if (allFilesAnalyzed) {
      setIsAnalysisComplete(true);
      setTimeout(() => setProgress(0), 500); // Hide bar after phase 1 is complete
      return;
    }

    const analyzeFiles = async () => {
      const filesToAnalyze = files.filter(
        (file) => file.software === undefined,
      );
      const total = files.length;
      let analyzedCount = total - filesToAnalyze.length;
      setProgress((analyzedCount / total) * 100);

      await Promise.all(
        filesToAnalyze.map(async (file) => {
          try {
            const fullContent = await file.fileObject.async("string");
            const headerContent = fullContent
              .split("\n")
              .slice(0, 10)
              .join("\n");
            const identifiedSoftware = await identifyFileType(headerContent);
            setFileSoftware(file.name, identifiedSoftware || "None");
          } catch (error) {
            console.error(`Could not analyze file ${file.name}:`, error);
            setFileSoftware(file.name, "None");
          }
          analyzedCount++;
          setProgress((analyzedCount / total) * 100);
        }),
      );
    };

    if (files.length > 0 && files.some((f) => f.software === undefined)) {
      analyzeFiles();
    }
  }, [files, files.length, setFileSoftware, setProgress]);

  const aggregatedBadges = useMemo(() => {
    // This memo remains unchanged.
    const stats: { [key: string]: { count: number; firstIndex: number } } = {};
    files.forEach((file, index) => {
      const software = file.software;
      if (software) {
        if (!stats[software]) {
          stats[software] = { count: 1, firstIndex: index };
        } else {
          stats[software].count++;
        }
      }
    });
    return Object.entries(stats)
      .map(([software, data]) => ({ software, ...data }))
      .sort((a, b) => a.firstIndex - b.firstIndex);
  }, [files]);

  // --- Phase 2: Processing Progress ---
  const handleProcessClick = async () => {
    startProcessing();

    const originalFilenames = files.map((f) => f.name);
    const sortedFilenames = await sortGerberFiles(originalFilenames);
    const primaryFile = files.find((f) => f.name === sortedFilenames[0]);
    const detectedPrimaryEda = primaryFile?.software;

    if (detectedPrimaryEda !== "Altium" && detectedPrimaryEda !== "KiCad") {
      alert(
        `Processing is currently only supported for Altium and KiCad projects. The primary type detected was "${detectedPrimaryEda || "None"}".`,
      );
      setProcessedFiles([], "None");
      setProgress(0);
      return;
    }
    const localRenameMap = await mapFilenames(
      originalFilenames,
      detectedPrimaryEda,
    );
    setRenameMap(localRenameMap); // Save map for later use by copy actions

    const newProcessedFiles = [];
    const total = files.length;
    let processedCount = 0;
    setProgress(0);

    for (const file of files) {
      let content = await file.fileObject.async("string");
      if (file.software === "Altium" || file.software === "KiCad") {
        content = await addGerberHeader(content);
      }
      if (file.software === "KiCad") {
        content = await processDCodes(content);
      }
      if (
        file.software === "Altium" ||
        file.software === "KiCad" ||
        file.software === "EasyEDA"
      ) {
        const isForeign = file.software !== "EasyEDA";
        content = await addFingerprint(content, isForeign);
      }

      newProcessedFiles.push({
        originalName: file.name,
        newName: localRenameMap.get(file.name) || file.name,
        content: content,
      });

      processedCount++;
      setProgress((processedCount / total) * 100);
    }

    // Get the guide text and add it as a new file to the processed list.
    const guideContent = await getOrderGuideText();
    newProcessedFiles.push({
      originalName: "PCB下单必读.txt",
      newName: "PCB下单必读.txt",
      content: guideContent,
    });

    setProcessedFiles(newProcessedFiles, detectedPrimaryEda);
    setTimeout(() => setProgress(0), 500);
  };

  const handleDownloadClick = async () => {
    // This function remains unchanged.
    if (!originalZipName || processedFiles.length === 0 || !primaryEda) return;

    const baseName = originalZipName.replace(/\.zip$/i, "");
    const edaSuffix = primaryEda === "Altium" ? "AD" : "Ki";
    const newZipName = `${baseName}-${edaSuffix}.zip`;

    const zip = new JSZip();
    processedFiles.forEach((file) => {
      zip.file(file.newName, file.content);
    });

    const blob = await zip.generateAsync({ type: "blob" });
    const link = document.createElement("a");
    link.href = URL.createObjectURL(blob);
    link.download = newZipName;
    document.body.appendChild(link);
    link.click();
    document.body.removeChild(link);
    URL.revokeObjectURL(link.href);
  };

  // Handler for the copy action on a file in the left panel.
  const handleCopyFile = async (file: GerberFile) => {
    try {
      const content = await file.fileObject.async("string");
      const newName = renameMap.get(file.name) || file.name;
      const processedFile: ProcessedGerberFile = {
        originalName: file.name,
        newName: newName,
        content: content, // Note: This copies the *original* content, not the processed one.
      };
      copyFileToProcessed(processedFile);
    } catch (error) {
      console.error(`Failed to copy file ${file.name}:`, error);
      alert(`Could not read and copy file: ${file.name}`);
    }
  };

  return (
    <div className="w-full max-w-6xl mx-auto grid grid-cols-1 md:grid-cols-2 gap-8">
      {/* Left Box */}
      <div className="relative">
        <div className="flex justify-between items-center mb-2">
          {/* Header remains the same */}
          <h2 className="text-lg font-semibold text-[var(--color-text)] flex items-center">
            <PackageSearch className="h-5 w-5 mr-2" />
            Gerber Files
            {aggregatedBadges.map(({ software, count }) => (
              <HeaderBadge key={software} software={software} count={count} />
            ))}
          </h2>
          <button
            onClick={handleProcessClick}
            disabled={!isAnalysisComplete || isProcessing}
            className="px-3 py-1 bg-[var(--color-accent)] text-white text-sm font-semibold rounded-md disabled:bg-gray-500 disabled:cursor-not-allowed hover:opacity-90 transition-opacity flex items-center gap-2"
          >
            {isProcessing ? (
              <LoaderCircle className="h-4 w-4 animate-spin" />
            ) : (
              "Process"
            )}
          </button>
        </div>
        <div className="p-4 border-2 border-dashed border-gray-500 rounded-lg h-96 flex flex-col">
          <ul className="flex-1 overflow-y-auto">
            {files.map((file) => (
              // Added group class for hover effects.
              <li
                key={file.name}
                className="group px-3 py-0.5 rounded transition-colors hover:bg-[var(--color-bg-alt)] flex justify-between items-center"
              >
                <span className="text-sm text-[var(--color-text)] truncate">
                  {file.name}
                </span>
                {/* Arrow button that appears on hover. */}
                <button
                  onClick={() => handleCopyFile(file)}
                  className="opacity-0 group-hover:opacity-100 transition-opacity text-gray-400 hover:text-[var(--color-accent)]"
                  title="Copy this file to the export list"
                  disabled={processedFiles.length === 0} // Disable if not yet processed
                >
                  <ArrowRight className="h-4 w-4" />
                </button>
              </li>
            ))}
          </ul>
        </div>
      </div>

      {/* Right Box */}
      <div className="relative">
        <div className="flex justify-between items-center mb-2">
          {/* Header remains the same */}
          <h2 className="text-lg font-semibold text-[var(--color-text)] flex items-center">
            <Archive className="h-5 w-5 mr-2" />
            Export Preview
          </h2>
          <button
            onClick={handleDownloadClick}
            disabled={processedFiles.length === 0 || isProcessing}
            className="px-3 py-1 bg-[var(--color-accent)] text-white text-sm font-semibold rounded-md disabled:bg-gray-500 disabled:cursor-not-allowed hover:opacity-90 transition-opacity flex items-center gap-2"
          >
            Download
          </button>
        </div>
        <div className="p-4 border-2 border-dashed border-gray-500 rounded-lg h-96 flex flex-col">
          {processedFiles.length > 0 ? (
            <ul className="flex-1 overflow-y-auto">
              {processedFiles.map((file) => (
                // Added group class and flex for new icon.
                <li
                  key={file.originalName}
                  className="group px-3 py-0.5 rounded transition-colors hover:bg-[var(--color-bg-alt)] flex justify-between items-center"
                >
                  <span className="text-sm text-[var(--color-text)] truncate">
                    {file.newName}
                  </span>
                  {/* Remove button that appears on hover. */}
                  <button
                    onClick={() => removeFileFromProcessed(file.originalName)}
                    className="opacity-0 group-hover:opacity-100 transition-opacity text-gray-400 hover:text-red-500"
                    title="Remove this file from the export list"
                  >
                    <X className="h-4 w-4" />
                  </button>
                </li>
              ))}
            </ul>
          ) : (
            <div className="flex-1 flex items-center justify-center text-center">
              <p className="text-[var(--color-subtext)]">
                {!isAnalysisComplete && files.length > 0 && !isProcessing
                  ? "Analyzing files..."
                  : ""}
                {isProcessing ? "Processing files..." : ""}
                {isAnalysisComplete &&
                !isProcessing &&
                processedFiles.length === 0
                  ? "Click 'Process' to begin."
                  : ""}
              </p>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

export default Workflow;
