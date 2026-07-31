// Do not edit manually! Code generated from ../../../crate-genotype-project-core/types/source_code.type

import { z } from "zod";

export const GtpSourceCodeHash = z.string().brand<"GtpSourceCodeHash">();

export type GtpSourceCodeHash = z.infer<typeof GtpSourceCodeHash>;

export const GtpSourceCode = z.object({
  content: z.string(),
  hash: GtpSourceCodeHash,
});

export type GtpSourceCode = z.infer<typeof GtpSourceCode>;
