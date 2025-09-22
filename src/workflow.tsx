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
  getGerberHeader,
  processDCodes,
  addFingerprint,
  sortGerberFiles,
  mapFilenames,
  getOrderGuideText,
  validateGerberFiles,
  type ValidationResult,
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
    layerCount,
  } = useWorkflowStore();

  const [isAnalysisComplete, setIsAnalysisComplete] = useState(false);
  const [renameMap, setRenameMap] = useState<Map<string, string>>(new Map());

  // --- Phase 1: Analysis Progress (Unchanged) ---
  useEffect(() => {
    const allFilesAnalyzed = files.every((f) => f.software !== undefined);
    if (allFilesAnalyzed) {
      setIsAnalysisComplete(true);
      setTimeout(() => setProgress(0), 500);
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
    // --- STRICT MODE GUARD ---
    // This flag check is crucial. If the component re-renders while this async
    // function is already running (a behavior React's Strict Mode can cause),
    // this guard prevents the entire processing logic from running a second time.
    if (isProcessing) {
      return;
    }
    // --- END GUARD ---

    startProcessing();

    const originalFilenames = files.map((f) => f.name);
    const sortedFilenames = await sortGerberFiles(originalFilenames);
    const primaryFile = files.find((f) => f.name === sortedFilenames[0]);
    const detectedPrimaryEda = primaryFile?.software;

    if (detectedPrimaryEda !== "Altium" && detectedPrimaryEda !== "KiCad") {
      alert(
        `Processing is currently only supported for Altium and KiCad projects. The primary type detected was "${detectedPrimaryEda || "None"}".`,
      );
      setProcessedFiles([], null, null);
      setProgress(0);
      return;
    }
    const localRenameMap = await mapFilenames(
      originalFilenames,
      detectedPrimaryEda,
    );
    setRenameMap(localRenameMap);

    const sharedHeader = await getGerberHeader();

    const newProcessedFiles = [];
    const total = files.length;
    let processedCount = 0;
    setProgress(0);

    for (const file of files) {
      let content = await file.fileObject.async("string");
      content = content.replace(/\r\n/g, "\n");

      if (file.software === "Altium" || file.software === "KiCad") {
        content = sharedHeader + content;
      }

      if (file.software === "KiCad") {
        content = await processDCodes(content);
      }
      if (
        file.software === "Altium" ||
        file.software === "KiCad" ||
        file.software === "EasyEDA"
      ) {
        content = await addFingerprint(content, false);
      }

      newProcessedFiles.push({
        originalName: file.name,
        newName: localRenameMap.get(file.name) || file.name,
        content: content,
      });

      processedCount++;
      setProgress((processedCount / total) * 100);
    }

    const finalFilenames = newProcessedFiles.map((f) => f.newName);
    const validationResult: ValidationResult =
      await validateGerberFiles(finalFilenames);

    if (!validationResult.is_valid) {
      const errorMessages = validationResult.errors.join("\n");
      alert(`Gerber file validation failed:\n\n${errorMessages}`);
      setProcessedFiles([], null, null);
      setProgress(0);
      validationResult.free();
      return;
    }

    if (validationResult.warnings.length > 0) {
      console.warn("Validation Warnings:");
      validationResult.warnings.forEach((w) => console.warn(`- ${w}`));
    }

    const guideContent = await getOrderGuideText();
    newProcessedFiles.push({
      originalName: "PCB下单必读.txt",
      newName: "PCB下单必读.txt",
      content: guideContent,
    });

    setProcessedFiles(
      newProcessedFiles,
      detectedPrimaryEda,
      validationResult.layer_count,
    );
    validationResult.free();
    setTimeout(() => setProgress(0), 500);
  };

  const handleDownloadClick = async () => {
    if (
      !originalZipName ||
      processedFiles.length === 0 ||
      !primaryEda ||
      layerCount === null
    )
      return;

    const baseName = originalZipName.replace(/\.zip$/i, "");
    const edaSuffix = primaryEda === "Altium" ? "AD" : "Ki";
    const layerSuffix = layerCount > 0 ? `-L${layerCount}` : "";
    const newZipName = `${baseName}-${edaSuffix}${layerSuffix}.zip`;

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

  const handleCopyFile = async (file: GerberFile) => {
    try {
      const content = await file.fileObject.async("string");
      const newName = renameMap.get(file.name) || file.name;
      const processedFile: ProcessedGerberFile = {
        originalName: file.name,
        newName: newName,
        content: content,
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
              <li
                key={file.name}
                className="group px-3 py-0.5 rounded transition-colors hover:bg-[var(--color-bg-alt)] flex justify-between items-center"
              >
                <span className="text-sm text-[var(--color-text)] truncate">
                  {file.name}
                </span>
                <button
                  onClick={() => handleCopyFile(file)}
                  className="opacity-0 group-hover:opacity-100 transition-opacity text-gray-400 hover:text-[var(--color-accent)]"
                  title="Copy this file to the export list"
                  disabled={processedFiles.length === 0}
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
                <li
                  key={file.originalName}
                  className="group px-3 py-0.5 rounded transition-colors hover:bg-[var(--color-bg-alt)] flex justify-between items-center"
                >
                  <span className="text-sm text-[var(--color-text)] truncate">
                    {file.newName}
                  </span>
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
