import { spawn } from "node:child_process";

export async function setup() {
  await new Promise<void>((resolve, reject) => {
    const child = spawn("mise", ["run", "//pkgs/npm-vscode-genotype:build:dev"]);

    child.on("error", reject);
    child.on("exit", (code) => {
      if (code === 0) resolve();
      else reject(new Error(`Build exited with code ${code}`));
    });
  });
}
