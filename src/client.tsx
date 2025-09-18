/* src/client.tsx */

import Footer from "./footer.tsx";

function App() {
  return (
    <main className="min-h-[100dvh] flex flex-col">
      <div className="flex-1 flex justify-center items-center">
        <h1 className="text-4xl font-bold">Hello, World!</h1>
      </div>
      <Footer />
    </main>
  );
}

export default App;
