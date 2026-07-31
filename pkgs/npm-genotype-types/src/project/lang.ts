// Do not edit manually! Code generated from ../../../crate-genotype-project-core/types/lang.type

import { z } from "zod";

export const GtpLang = z.union([z.literal("py"), z.literal("rs"), z.literal("ts")]);

export type GtpLang = z.infer<typeof GtpLang>;
