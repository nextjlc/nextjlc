/* src/client.tsx */

import Footer from "./footer.tsx";
import UploadZone from "./upload.tsx";
import Workflow from "./workflow.tsx";
import ProgressBar from "./progress-bar.tsx";
import { useWorkflowStore } from "./editfile";

function App() {
  const { workflowState } = useWorkflowStore();

  return (
    <main className="min-h-[100dvh] flex flex-col">
      <ProgressBar />
      <div className="flex-1 flex justify-center items-center p-4">
        {workflowState === "upload" ? <UploadZone /> : <Workflow />}
      </div>
      <Footer />
    </main>
  );
}

export default App;
