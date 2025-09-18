/* src/workflow.tsx */

import { useWorkflowStore } from "./editfile";
import { PackageSearch, Archive } from "lucide-react";

function Workflow() {
  const { files } = useWorkflowStore();

  return (
    <div className="w-full max-w-6xl mx-auto grid grid-cols-1 md:grid-cols-2 gap-8">
      <div>
        <h2 className="text-lg font-semibold text-[var(--color-text)] mb-2 flex items-center">
          <PackageSearch className="h-5 w-5 mr-2" />
          Gerber Files
        </h2>
        <div className="p-4 border-2 border-dashed border-gray-500 rounded-lg h-96 flex flex-col">
          <ul className="flex-1 overflow-y-auto">
            {files.map((fileName, index) => (
              <li
                key={index}
                className="px-3 py-0.5 rounded transition-colors hover:bg-[var(--color-bg-alt)]"
              >
                <span className="text-sm text-[var(--color-text)] truncate">
                  {fileName}
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
