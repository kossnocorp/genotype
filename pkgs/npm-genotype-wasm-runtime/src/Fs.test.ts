import { describe, expect, it } from "vitest";
import { GtwmFs } from "./Fs";

describe("GtwmFs", () => {
  describe("findFile", () => {
    it.each([undefined, "", ".", "src/nested", "/", "/workspace/src"])(
      "missing file returns null from %j",
      (basePath) => {
        expect(createFs().findFile("genotype.toml", basePath)).toBeNull();
      },
    );

    it("finds a file in the default directory", () => {
      const fs = createFs({ "genotype.toml": "" });
      expect(fs.findFile("genotype.toml")).toBe("genotype.toml");
    });

    it("finds the nearest ancestor's file", () => {
      const fs = createFs({ "genotype.toml": "", "src/genotype.toml": "" });
      expect(fs.findFile("genotype.toml", "src/nested")).toBe("src/genotype.toml");
    });

    it("checks relative and absolute roots before stopping", () => {
      const fs = createFs({ "genotype.toml": "", "/genotype.toml": "" });
      expect(fs.findFile("genotype.toml", "src/nested")).toBe("genotype.toml");
      expect(fs.findFile("genotype.toml", "/workspace/src")).toBe("/genotype.toml");
    });
  });
});

function createFs(files = {}) {
  const fs = new GtwmFs({ files });
  const isFile = fs.isFile.bind(fs);
  let lookups = 0;
  fs.isFile = (path) => {
    expect(++lookups, "file search must terminate").toBeLessThanOrEqual(20);
    return isFile(path);
  };
  return fs;
}
