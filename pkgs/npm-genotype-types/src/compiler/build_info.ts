// Do not edit manually! Code generated from ../../../crate-genotype-compiler/types/build_info.type

import { z } from "zod";
import { GtpBuildInfoSrc, GtpBuildInfoDist } from "@genotype-lang/types";

export const GtpBuildInfo = z.object({
  src: GtpBuildInfoSrc,
  dist: GtpBuildInfoDist,
});

export type GtpBuildInfo = z.infer<typeof GtpBuildInfo>;
