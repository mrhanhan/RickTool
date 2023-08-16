import {defineConfig} from "vite";
import react from "@vitejs/plugin-react";

const mobile =
    process.env.TAURI_PLATFORM === "android" ||
    process.env.TAURI_PLATFORM === "ios";

// https://vitejs.dev/config/
// @ts-ignore
export default defineConfig(async () => ({
    plugins: [react()],
    clearScreen: false,
    server: {
        host: '0.0.0.0',
        port: 1420,
        strictPort: true,
    },
    envPrefix: ["VITE_", "TAURI_"],
    build: {
        target: process.env.TAURI_PLATFORM == "windows" ? "chrome105" : "safari13",
        minify: !process.env.TAURI_DEBUG ? "esbuild" : false,
        sourcemap: !!process.env.TAURI_DEBUG,
    },
    css: {
        postcss: {},
        preprocessorOptions: {
            less: true
        }
    }
}));
