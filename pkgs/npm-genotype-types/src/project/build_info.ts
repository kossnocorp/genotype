// Do not edit manually! Code generated from ../../../crate-genotype-project-core/types/build_info.type

import { z } from "zod";
import { GtModuleId } from "@genotype-lang/types";
import { GtpSourceCodeHash } from "./source_code.js";

/** Path relative to the config dir */
export const GtpBuildInfoPath = z.string().brand<"GtpBuildInfoPath">();

/** Path relative to the config dir */
export type GtpBuildInfoPath = z.infer<typeof GtpBuildInfoPath>;

export const GtpBuildInfoSrcModule = z.object({
  id: GtModuleId,
  hash: GtpSourceCodeHash,
  deps: z.array(GtModuleId),
});

export type GtpBuildInfoSrcModule = z.infer<typeof GtpBuildInfoSrcModule>;

export const GtpBuildInfoSrcModules = z.record(GtpBuildInfoPath, GtpBuildInfoSrcModule);

export type GtpBuildInfoSrcModules = z.infer<typeof GtpBuildInfoSrcModules>;

export const GtpBuildInfoSrc = z.object({
  config_hash: GtpSourceCodeHash,
  modules: GtpBuildInfoSrcModules,
});

export type GtpBuildInfoSrc = z.infer<typeof GtpBuildInfoSrc>;

export const GtpBuildInfoDistFile = z.object({
  hash: GtpSourceCodeHash,
  srcId: z.union([GtModuleId, z.undefined()]).optional(),
});

export type GtpBuildInfoDistFile = z.infer<typeof GtpBuildInfoDistFile>;

export const GtpBuildInfoDistFiles = z.record(GtpBuildInfoPath, GtpBuildInfoDistFile);

export type GtpBuildInfoDistFiles = z.infer<typeof GtpBuildInfoDistFiles>;

export const GtpBuildInfoDist = z.object({
  ts: z.union([GtpBuildInfoDistFiles, z.undefined()]).optional(),
  rs: z.union([GtpBuildInfoDistFiles, z.undefined()]).optional(),
  py: z.union([GtpBuildInfoDistFiles, z.undefined()]).optional(),
});

export type GtpBuildInfoDist = z.infer<typeof GtpBuildInfoDist>;

export const GtpBuildInfo = z.object({
  src: GtpBuildInfoSrc,
  dist: GtpBuildInfoDist,
});

export type GtpBuildInfo = z.infer<typeof GtpBuildInfo>;
