/* src/workflow.tsx */

import { useEffect, useMemo } from "react";
import { useWorkflowStore } from "./editfile";
import { PackageSearch, Archive } from "lucide-react";
import { identifyFileType } from "./filetype";

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
  const { files, setFileSoftware } = useWorkflowStore();

  useEffect(() => {
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

  return (
    <div className="w-full max-w-6xl mx-auto grid grid-cols-1 md:grid-cols-2 gap-8">
      <div>
        <h2 className="text-lg font-semibold text-[var(--color-text)] mb-2 flex items-center">
          <PackageSearch className="h-5 w-5 mr-2" />
          Gerber Files
          {aggregatedBadges.map(({ software, count }) => (
            <HeaderBadge key={software} software={software} count={count} />
          ))}
        </h2>
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
