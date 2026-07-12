import { z } from "zod";

export const GtMetaPathsLang = z.object({
  pkg: z.string(),
  src: z.string(),
});

export type GtMetaPathsLang = z.infer<typeof GtMetaPathsLang>;

export const GtMetaPaths = z.object({
  src: z.string(),
  dist: z.string(),
  ts: z.union([GtMetaPathsLang, z.undefined()]).optional(),
  rs: z.union([GtMetaPathsLang, z.undefined()]).optional(),
  py: z.union([GtMetaPathsLang, z.undefined()]).optional(),
});

export type GtMetaPaths = z.infer<typeof GtMetaPaths>;

export const GtMetaTiming = z.object({
  totalMs: z.number(),
  loadProjectMs: z.number(),
  loadModulesMs: z.number(),
  compileMs: z.number(),
});

export type GtMetaTiming = z.infer<typeof GtMetaTiming>;

export const GtMetaModule = z.object({
  source: z.string(),
  ts: z.union([z.string(), z.undefined()]).optional(),
  rs: z.union([z.string(), z.undefined()]).optional(),
  py: z.union([z.string(), z.undefined()]).optional(),
});

export type GtMetaModule = z.infer<typeof GtMetaModule>;

export const GtMeta = z.object({
  exitCode: z.number(),
  paths: GtMetaPaths,
  timing: GtMetaTiming,
  modules: z.array(GtMetaModule),
});

export type GtMeta = z.infer<typeof GtMeta>;
