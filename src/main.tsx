/* src/main.tsx */

import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./client.tsx";
import "./style.css";

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
