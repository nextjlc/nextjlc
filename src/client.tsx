/* src/client.tsx */

import Footer from "./footer.tsx";
import UploadZone from "./upload.tsx";
import Workflow from "./workflow.tsx";
import { useWorkflowStore } from "./editfile";

function App() {
  // Subscribe to the workflowState from our global store
  const { workflowState } = useWorkflowStore();

  return (
    <main className="min-h-[100dvh] flex flex-col">
      <div className="flex-1 flex justify-center items-center p-4">
        {/* Conditionally render the component based on the current state */}
        {workflowState === "upload" ? <UploadZone /> : <Workflow />}
      </div>
      <Footer />
    </main>
  );
}

export default App;
