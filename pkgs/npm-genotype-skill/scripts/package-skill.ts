#!/usr/bin/env node

/**
 * Packages the canonical skill for GitHub Releases.
 */

import cp from "node:child_process";
import crypto from "node:crypto";
import fs from "node:fs/promises";
import path from "node:path";
import util from "node:util";
import { Mdx, type Skill } from "../src/index.ts";
import { generateSkill, skillDir } from "./generate-skill.ts";

const execFile = util.promisify(cp.execFile);

if (import.meta.main) {
  const output = process.argv[2];
  if (!output) throw new Error("package-skill.ts OUTPUT_DIR");
  await packageSkill(output);
}

export async function packageSkill(outDirArg: string): Promise<void> {
  // Check generated skill files
  await generateSkill(true);

  const outDir = path.resolve(outDirArg);
  await fs.mkdir(outDir, { recursive: true });

  const digest = await zipFiles(skillDir, outDir);
  const description = await readDescription(skillDir);

  const skill: Skill.Manifest = {
    name: "genotype",
    description,
    digest,
  };

  const skillJsonPath = path.join(outDir, "genotype-skill.json");
  await fs.writeFile(skillJsonPath, JSON.stringify(skill, null, 2));
}

async function zipFiles(skillDir: string, outDir: string) {
  const skillFilePaths = await fs.readdir(skillDir);
  skillFilePaths.sort();

  const zipPath = path.join(outDir, "genotype-skill.zip");
  await fs.rm(zipPath, { force: true });
  await execFile("zip", ["-q", "-X", zipPath, ...skillFilePaths], { cwd: skillDir });

  const zipContent = await fs.readFile(zipPath);
  const zipSha = crypto.createHash("sha256").update(zipContent).digest("hex");
  return `sha256:${zipSha}`;
}

async function readDescription(skillDir: string): Promise<string> {
  const skillMdPath = path.join(skillDir, "SKILL.md");
  const skillMd = await fs.readFile(skillMdPath, "utf8");

  const description = Mdx.parseFrontmatter(skillMd)?.description;
  if (typeof description !== "string" || !description) throw new Error("Missing skill description");

  return description;
}
