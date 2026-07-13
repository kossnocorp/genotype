import { z } from "zod";

export const GtcMetaNew = z.object({});

export type GtcMetaNew = z.infer<typeof GtcMetaNew>;

export const GtcMetaLoadedProjectPaths = z.object({
  src: z.string(),
});

export type GtcMetaLoadedProjectPaths = z.infer<typeof GtcMetaLoadedProjectPaths>;

export const GtcMetaLoadedProject = z.object({
  paths: GtcMetaLoadedProjectPaths,
});

export type GtcMetaLoadedProject = z.infer<typeof GtcMetaLoadedProject>;

export const GtcMetaLoadedModules = z.object({
  paths: GtcMetaLoadedProjectPaths,
  modules: z.array(z.string()),
});

export type GtcMetaLoadedModules = z.infer<typeof GtcMetaLoadedModules>;

export const GtcMetaCompiledPathsLang = z.object({
  pkg: z.string(),
  src: z.string(),
});

export type GtcMetaCompiledPathsLang = z.infer<typeof GtcMetaCompiledPathsLang>;

export const GtcMetaCompiledPaths = z.object({
  src: z.string(),
  dist: z.string(),
  ts: z.union([GtcMetaCompiledPathsLang, z.undefined()]).optional(),
  rs: z.union([GtcMetaCompiledPathsLang, z.undefined()]).optional(),
  py: z.union([GtcMetaCompiledPathsLang, z.undefined()]).optional(),
});

export type GtcMetaCompiledPaths = z.infer<typeof GtcMetaCompiledPaths>;

export const GtcMetaCompiledModule = z.object({
  source: z.string(),
  ts: z.union([z.string(), z.undefined()]).optional(),
  rs: z.union([z.string(), z.undefined()]).optional(),
  py: z.union([z.string(), z.undefined()]).optional(),
});

export type GtcMetaCompiledModule = z.infer<typeof GtcMetaCompiledModule>;

export const GtcMetaCompiled = z.object({
  exitCode: z.number(),
  paths: GtcMetaCompiledPaths,
  modules: z.array(GtcMetaCompiledModule),
});

export type GtcMetaCompiled = z.infer<typeof GtcMetaCompiled>;
