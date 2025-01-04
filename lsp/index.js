const { join } = require("path");
const { existsSync } = require("fs");

module.exports.getBinPath = function getBinPath(
  contextPath = import.meta.dirname
) {
  const variation = `${process.platform}-${process.arch}`;
  const binName = process.platform === "win32" ? "gts.exe" : "gts";
  const binPath = join(
    contextPath,
    "node_modules",
    "@genotype-lsp",
    variation,
    "bin",
    binName
  );

  if (!existsSync(binPath))
    throw new Error(`Genotype LSP binary not found at: ${binPath}`);

  return binPath;
};
