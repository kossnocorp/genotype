import z from "zod";

export abstract class Skill {
  static Manifest = z.object({
    name: z.string(),
    description: z.string(),
    digest: z.string(),
  });
}

export namespace Skill {
  export type Manifest = z.infer<typeof Skill.Manifest>;
}
