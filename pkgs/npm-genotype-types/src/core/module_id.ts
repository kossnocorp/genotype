import { z } from "zod";

/** Module identifier. */
export const GtModuleId = z.string().brand<"GtModuleId">();

/** Module identifier. */
export type GtModuleId = z.infer<typeof GtModuleId>;
