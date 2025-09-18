/* vite.config.ts */

import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import { execSync } from "child_process";
import wasm from "vite-plugin-wasm";

const gitCommitHash = execSync("git rev-parse --short HEAD").toString().trim();

// https://vite.dev/config/
export default defineConfig({
  plugins: [react(), wasm()],
  define: {
    __GIT_HASH__: JSON.stringify(gitCommitHash),
  },
});
