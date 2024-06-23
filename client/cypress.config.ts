import { defineConfig } from "cypress";

export default defineConfig({
  e2e: {
    video: true,
    supportFile: false,
    baseUrl: "http://localhost:8000",
    setupNodeEvents(on, config) {
      // implement node event listeners here
    },
  },
});
