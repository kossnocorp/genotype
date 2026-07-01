import { z } from "zod";

export const GtpFormatterGenericBase = z.object({
  cmd: z.string(),
  args: z.union([z.array(z.string()), z.undefined()]).optional(),
});

export type GtpFormatterGenericBase = z.infer<typeof GtpFormatterGenericBase>;

export const GtpFormatterShell = GtpFormatterGenericBase.extend({
  kind: z.literal("shell"),
});

export type GtpFormatterShell = z.infer<typeof GtpFormatterShell>;

export const GtpFormatterExecutorKindNode = z.union([
  z.literal("pnpm"),
  z.literal("pnx"),
  z.literal("npm"),
  z.literal("npx"),
  z.literal("bun"),
  z.literal("bunx"),
]);

export type GtpFormatterExecutorKindNode = z.infer<typeof GtpFormatterExecutorKindNode>;

export const GtpFormatterPresetNodeBase = z.object({
  via: z.union([GtpFormatterExecutorKindNode, z.undefined()]).optional(),
  args: z.union([z.array(z.string()), z.undefined()]).optional(),
});

export type GtpFormatterPresetNodeBase = z.infer<typeof GtpFormatterPresetNodeBase>;

export const GtpFormatterPresetOxfmt = GtpFormatterPresetNodeBase.extend({
  kind: z.literal("oxfmt"),
});

export type GtpFormatterPresetOxfmt = z.infer<typeof GtpFormatterPresetOxfmt>;

export const GtpFormatterPresetPrettier = GtpFormatterPresetNodeBase.extend({
  kind: z.literal("prettier"),
});

export type GtpFormatterPresetPrettier = z.infer<typeof GtpFormatterPresetPrettier>;

export const GtpFormatterExecutorKindPython = z.union([
  z.literal("uv"),
  z.literal("poetry"),
  z.literal("pipx"),
]);

export type GtpFormatterExecutorKindPython = z.infer<typeof GtpFormatterExecutorKindPython>;

export const GtpFormatterExecutorKind = z.union([
  z.literal("cargo"),
  GtpFormatterExecutorKindNode,
  GtpFormatterExecutorKindPython,
]);

export type GtpFormatterExecutorKind = z.infer<typeof GtpFormatterExecutorKind>;

export const GtpFormatterExecutor = GtpFormatterGenericBase.extend({
  kind: GtpFormatterExecutorKind,
});

export type GtpFormatterExecutor = z.infer<typeof GtpFormatterExecutor>;

export const GtpFormatterPresetPythonBase = z.object({
  via: z.union([GtpFormatterExecutorKindPython, z.undefined()]).optional(),
  args: z.union([z.array(z.string()), z.undefined()]).optional(),
});

export type GtpFormatterPresetPythonBase = z.infer<typeof GtpFormatterPresetPythonBase>;

export const GtpFormatterPresetRuff = GtpFormatterPresetPythonBase.extend({
  kind: z.literal("ruff"),
});

export type GtpFormatterPresetRuff = z.infer<typeof GtpFormatterPresetRuff>;

export const GtpFormatter = z.union([
  GtpFormatterShell,
  GtpFormatterExecutor,
  GtpFormatterPresetOxfmt,
  GtpFormatterPresetPrettier,
  GtpFormatterPresetRuff,
]);

export type GtpFormatter = z.infer<typeof GtpFormatter>;

export const GtpFormatterCmd = z.object({
  cmd: z.string(),
  args: z.array(z.string()),
});

export type GtpFormatterCmd = z.infer<typeof GtpFormatterCmd>;
