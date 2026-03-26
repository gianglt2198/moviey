import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [react()],
  //   build: {
  //     target: "ES2020",
  //     minify: "terser",
  //     terserOptions: {
  //       compress: {
  //         drop_console: true,
  //       },
  //     },
  //     rollupOptions: {
  //       output: {
  //         manualChunks: {
  //           "video-player": ["video.js"],
  //           "http-client": ["axios"],
  //         },
  //       },
  //     },
  //   },
  server: {
    proxy: {
      "/api": {
        target: "http://localhost:3000",
        changeOrigin: true,
      },
    },
  },
});
