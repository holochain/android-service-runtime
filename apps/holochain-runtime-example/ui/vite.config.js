import { defineConfig } from "vite";

// Static frontend bundled into the Tauri app. Output to ./dist, which the
// Tauri config points at via `frontendDist`.
export default defineConfig({
  build: {
    target: "esnext",
    emptyOutDir: true,
  },
});
