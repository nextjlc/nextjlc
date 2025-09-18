/* src/client.tsx */

import Footer from "./footer.tsx";
import UploadZone from "./upload.tsx"; // Import the new component

function App() {
  return (
    <main className="min-h-[100dvh] flex flex-col">
      {/* 
        The main content area now holds the UploadZone component.
        The surrounding layout classes remain the same to keep it centered.
      */}
      <div className="flex-1 flex justify-center items-center p-4">
        <UploadZone />
      </div>
      <Footer />
    </main>
  );
}

export default App;
