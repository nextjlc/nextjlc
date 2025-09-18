/* src/workflow.tsx */

import { useEffect } from "react";
import { useWorkflowStore } from "./editfile";
import { PackageSearch, Archive } from "lucide-react";
import { identifyFileType } from "./filetype";

const SoftwareBadge = ({ software }: { software?: string }) => {
  if (!software) return null; // Don't render anything if software is not identified

  const styles = {
    Altium: "bg-red-500/80 text-white",
    KiCad: "bg-blue-500/80 text-white",
    EasyEDA: "bg-sky-400/80 text-white",
  };

  const style = styles[software as keyof typeof styles] || "";

  return (
    <span className={`px-2 py-0.5 rounded-full text-xs font-medium ${style}`}>
      {software}
    </span>
  );
};

function Workflow() {
  // Get both the files and the action to update them
  const { files, setFileSoftware } = useWorkflowStore();

  // This effect runs when the 'files' array changes (i.e., after upload)
  useEffect(() => {
    // Asynchronously process each file
    const analyzeFiles = async () => {
      for (const file of files) {
        // Only process files that haven't been analyzed yet
        if (!file.software) {
          try {
            // Read the content of the file from the JSZip object
            const content = await file.fileObject.async("string");
            // Call our Wasm wrapper to identify the software
            const identifiedSoftware = await identifyFileType(content);
            if (identifiedSoftware) {
              // If found, update the global state for this specific file
              setFileSoftware(file.name, identifiedSoftware);
            }
          } catch (error) {
            console.error(`Could not analyze file ${file.name}:`, error);
          }
        }
      }
    };

    analyzeFiles();
  }, [files, setFileSoftware]); // Dependency array ensures this runs at the right time

  return (
    <div className="w-full max-w-6xl mx-auto grid grid-cols-1 md:grid-cols-2 gap-8">
      <div>
        <h2 className="text-lg font-semibold text-[var(--color-text)] mb-2 flex items-center">
          <PackageSearch className="h-5 w-5 mr-2" />
          Gerber Files
        </h2>
        <div className="p-4 border-2 border-dashed border-gray-500 rounded-lg h-96 flex flex-col">
          <ul className="flex-1 overflow-y-auto">
            {files.map((file) => (
              <li
                key={file.name}
                className="px-3 py-0.5 rounded transition-colors hover:bg-[var(--color-bg-alt)] flex justify-between items-center"
              >
                <span className="text-sm text-[var(--color-text)] truncate">
                  {file.name}
                </span>
                {/* Render the badge using the software property from our state */}
                <SoftwareBadge software={file.software} />
              </li>
            ))}
          </ul>
        </div>
      </div>

      <div>
        <h2 className="text-lg font-semibold text-[var(--color-text)] mb-2 flex items-center">
          <Archive className="h-5 w-5 mr-2" />
          Export Preview
        </h2>
        <div className="p-4 border-2 border-dashed border-gray-500 rounded-lg h-96 flex flex-col">
          <div className="flex-1 flex items-center justify-center text-center">
            <p className="text-[var(--color-subtext)]">
              This is where the next step of the process will happen.
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}

export default Workflow;
