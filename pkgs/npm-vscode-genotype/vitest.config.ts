import { defineConfig } from "vitest/config";

export default defineConfig({
  test: {
    passWithNoTests: true,

    projects: [
      {
        test: {
          name: "unit",
          include: ["src/**/*.test.ts"],
        },
      },

      {
        test: {
          name: "system",
          include: ["tests/system/**/*.test.ts"],
          globalSetup: ["./tests/system/globalSetup.ts"],
        },
      },
    ],
  },
});
