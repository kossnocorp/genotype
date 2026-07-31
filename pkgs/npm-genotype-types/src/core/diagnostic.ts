import { z } from "zod";

export const GtDiagnosticKind = z.union([z.literal("error"), z.literal("warning"), z.literal("success"), z.literal("info")]);

export type GtDiagnosticKind = z.infer<typeof GtDiagnosticKind>;

export const GtDiagnosticContentBase = z.object({
  title: z.string()
});

export type GtDiagnosticContentBase = z.infer<typeof GtDiagnosticContentBase>;

export const GtDiagnosticContentReport = GtDiagnosticContentBase.extend({
  report: z.string()
});

export type GtDiagnosticContentReport = z.infer<typeof GtDiagnosticContentReport>;

export const GtDiagnosticContentMessageBodySingle = z.string();

export type GtDiagnosticContentMessageBodySingle = z.infer<typeof GtDiagnosticContentMessageBodySingle>;

export const GtDiagnosticContentMessageBodyMulti = z.array(z.string());

export type GtDiagnosticContentMessageBodyMulti = z.infer<typeof GtDiagnosticContentMessageBodyMulti>;

export const GtDiagnosticContentMessageBody = z.union([GtDiagnosticContentMessageBodySingle, GtDiagnosticContentMessageBodyMulti]);

export type GtDiagnosticContentMessageBody = z.infer<typeof GtDiagnosticContentMessageBody>;

export const GtDiagnosticContentMessage = GtDiagnosticContentBase.extend({
  body: z.union([GtDiagnosticContentMessageBody, z.undefined()]).optional()
});

export type GtDiagnosticContentMessage = z.infer<typeof GtDiagnosticContentMessage>;

export const GtDiagnosticContent = z.union([GtDiagnosticContentMessage, GtDiagnosticContentReport]);

export type GtDiagnosticContent = z.infer<typeof GtDiagnosticContent>;

export const GtDiagnostic = z.object({
  kind: GtDiagnosticKind,
  content: GtDiagnosticContent
});

export type GtDiagnostic = z.infer<typeof GtDiagnostic>;
