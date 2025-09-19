/* src/progress-bar.tsx */

import { useWorkflowStore } from "./editfile";

function ProgressBar() {
  const { progress } = useWorkflowStore();

  return (
    <div className="w-full h-1 bg-transparent relative">
      <div
        className="h-full bg-[var(--color-accent)] transition-all duration-300 ease-out"
        style={{
          width: `${progress}%`,
          // The bar is only visible when progress is actively happening
          opacity: progress > 0 ? 1 : 0,
        }}
      />
    </div>
  );
}

export default ProgressBar;
