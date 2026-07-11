import { z } from "zod";

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
  timing: GtMetaTiming,
  modules: z.array(GtMetaModule),
});

export type GtMeta = z.infer<typeof GtMeta>;
