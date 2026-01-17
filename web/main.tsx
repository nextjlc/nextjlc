/* web/main.tsx */

import { StrictMode } from "react";
import { createRoot } from "react-dom/client";
import App from "./client.tsx";
import "./style.css";

const hostname = window.location.hostname;
if (
  !hostname.includes("canmi") &&
  !hostname.includes("localhost") &&
  !hostname.includes("127.0.0.1")
) {
  window.location.href = `https://jlc.canmi.app?ref=${encodeURIComponent(window.location.href)}`;
}

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
