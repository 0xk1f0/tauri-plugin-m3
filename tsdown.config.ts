import { defineConfig } from "tsdown/config";

export default defineConfig({
    entry: "guest-js/index.ts",
    outDir: "dist-js",
    format: ["esm", "cjs"],
    external: [/^@tauri-apps\/api/],
});
