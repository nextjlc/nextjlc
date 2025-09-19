/* src/workflow.tsx */

import { useEffect, useMemo, useState } from "react";
import { useWorkflowStore } from "./editfile";
import { PackageSearch, Archive, LoaderCircle, Download } from "lucide-react";
import JSZip from "jszip";
import {
  identifyFileType,
  addGerberHeader,
  processDCodes,
  addFingerprint,
  sortGerberFiles,
  mapFilenames,
} from "./filetype";

const HeaderBadge = ({
  software,
  count,
}: {
  software: string;
  count: number;
}) => {
  if (!software || count === 0) return null;

  const styles = {
    Altium: {
      badge: "bg-red-500/80 text-white",
      count: "bg-red-700 text-white",
    },
    KiCad: {
      badge: "bg-blue-500/80 text-white",
      count: "bg-blue-700 text-white",
    },
    EasyEDA: {
      badge: "bg-sky-400/80 text-white",
      count: "bg-sky-600 text-white",
    },
    None: {
      badge: "bg-gray-500/80 text-white",
      count: "bg-gray-700 text-white",
    },
  };

  const selectedStyle = styles[software as keyof typeof styles] || styles.None;

  return (
    <span
      className={`relative ml-2 px-2 py-0.5 rounded-full text-xs font-medium ${selectedStyle.badge}`}
    >
      {software}
      <span
        className={`absolute -top-1.5 -right-1.5 flex items-center justify-center
                  w-4 h-4 rounded-full text-[10px] font-bold shadow
                  ${selectedStyle.count}`}
      >
        {count}
      </span>
    </span>
  );
};

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
  } = useWorkflowStore();

  const [isAnalysisComplete, setIsAnalysisComplete] = useState(false);

  useEffect(() => {
    const allFilesAnalyzed = files.every((f) => f.software !== undefined);
    if (allFilesAnalyzed) {
      setIsAnalysisComplete(true);
      return;
    }
    const analyzeFiles = async () => {
      const filesToAnalyze = files.filter(
        (file) => file.software === undefined,
      );
      for (const file of filesToAnalyze) {
        try {
          const content = await file.fileObject.async("string");
          const identifiedSoftware = await identifyFileType(content);
          setFileSoftware(file.name, identifiedSoftware || "None");
        } catch (error) {
          console.error(`Could not analyze file ${file.name}:`, error);
          setFileSoftware(file.name, "None");
        }
      }
    };
    if (files.some((f) => f.software === undefined)) {
      analyzeFiles();
    }
  }, [files, setFileSoftware]);

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
      return;
    }

    const renameMap = await mapFilenames(originalFilenames, detectedPrimaryEda);
    const newProcessedFiles = await Promise.all(
      files.map(async (file) => {
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
        return {
          originalName: file.name,
          newName: renameMap.get(file.name) || file.name,
          content: content,
        };
      }),
    );
    setProcessedFiles(newProcessedFiles, detectedPrimaryEda);
  };

  const handleDownloadClick = async () => {
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

  return (
    <div className="w-full max-w-6xl mx-auto grid grid-cols-1 md:grid-cols-2 gap-8">
      {/* --- Left Box --- */}
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
                className="px-3 py-0.5 rounded transition-colors hover:bg-[var(--color-bg-alt)]"
              >
                <span className="text-sm text-[var(--color-text)] truncate">
                  {file.name}
                </span>
              </li>
            ))}
          </ul>
        </div>
      </div>

      {/* --- Right Box --- */}
      <div className="relative">
        <div className="flex justify-between items-center mb-2">
          <h2 className="text-lg font-semibold text-[var(--color-text)] flex items-center">
            <Archive className="h-5 w-5 mr-2" />
            Export Preview
          </h2>
          <button
            onClick={handleDownloadClick}
            disabled={processedFiles.length === 0 || isProcessing}
            className="px-3 py-1 bg-green-600 text-white text-sm font-semibold rounded-md disabled:bg-gray-500 disabled:cursor-not-allowed hover:opacity-90 transition-opacity flex items-center gap-2"
          >
            <Download className="h-4 w-4" />
            Download
          </button>
        </div>
        <div className="p-4 border-2 border-dashed border-gray-500 rounded-lg h-96 flex flex-col">
          {processedFiles.length > 0 ? (
            <ul className="flex-1 overflow-y-auto">
              {processedFiles.map((file) => (
                <li
                  key={file.originalName}
                  className="px-3 py-0.5 rounded transition-colors hover:bg-[var(--color-bg-alt)]"
                >
                  <span className="text-sm text-[var(--color-text)] truncate">
                    {file.newName}
                  </span>
                </li>
              ))}
            </ul>
          ) : (
            <div className="flex-1 flex items-center justify-center text-center">
              <p className="text-[var(--color-subtext)]">
                {isProcessing
                  ? "Processing files..."
                  : "Processed files will appear here."}
              </p>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

export default Workflow;
