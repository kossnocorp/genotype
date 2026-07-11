import { z } from "zod";
import { GtDiagnostic, GtMeta, GtpFormatterCmd } from "@genotype-lang/types";

export const GtbRemoteBackendRequestGlobFiles = z.object({
  kind: z.literal("glob-files"),
  path: z.string(),
});

export type GtbRemoteBackendRequestGlobFiles = z.infer<typeof GtbRemoteBackendRequestGlobFiles>;

export const GtbRemoteBackendRequestResponseGlobFiles = z.object({
  kind: z.literal("glob-files"),
  paths: z.array(z.string()),
});

export type GtbRemoteBackendRequestResponseGlobFiles = z.infer<
  typeof GtbRemoteBackendRequestResponseGlobFiles
>;

export const GtbRemoteBackendRequestReadFile = z.object({
  kind: z.literal("read-file"),
  path: z.string(),
});

export type GtbRemoteBackendRequestReadFile = z.infer<typeof GtbRemoteBackendRequestReadFile>;

export const GtbRemoteBackendRequestResponseReadFile = z.object({
  kind: z.literal("read-file"),
  content: z.string(),
});

export type GtbRemoteBackendRequestResponseReadFile = z.infer<
  typeof GtbRemoteBackendRequestResponseReadFile
>;

export const GtbRemoteBackendRequestFileExists = z.object({
  kind: z.literal("file-exists"),
  path: z.string(),
});

export type GtbRemoteBackendRequestFileExists = z.infer<typeof GtbRemoteBackendRequestFileExists>;

export const GtbRemoteBackendRequestResponseFileExists = z.object({
  kind: z.literal("file-exists"),
  exists: z.boolean(),
});

export type GtbRemoteBackendRequestResponseFileExists = z.infer<
  typeof GtbRemoteBackendRequestResponseFileExists
>;

export const GtbRemoteBackendRequestIsFile = z.object({
  kind: z.literal("is-file"),
  path: z.string(),
});

export type GtbRemoteBackendRequestIsFile = z.infer<typeof GtbRemoteBackendRequestIsFile>;

export const GtbRemoteBackendRequestResponseIsFile = z.object({
  kind: z.literal("is-file"),
  isFile: z.boolean(),
});

export type GtbRemoteBackendRequestResponseIsFile = z.infer<
  typeof GtbRemoteBackendRequestResponseIsFile
>;

export const GtbRemoteBackendRequestFindFile = z.object({
  kind: z.literal("find-file"),
  fileName: z.string(),
});

export type GtbRemoteBackendRequestFindFile = z.infer<typeof GtbRemoteBackendRequestFindFile>;

export const GtbRemoteBackendRequestResponseFindFile = z.object({
  kind: z.literal("find-file"),
  path: z.string(),
});

export type GtbRemoteBackendRequestResponseFindFile = z.infer<
  typeof GtbRemoteBackendRequestResponseFindFile
>;

export const GtbRemoteBackendRequestReportDiagnostic = z.object({
  kind: z.literal("report-diagnostic"),
  diagnostic: GtDiagnostic,
});

export type GtbRemoteBackendRequestReportDiagnostic = z.infer<
  typeof GtbRemoteBackendRequestReportDiagnostic
>;

export const GtbRemoteBackendRequestResponseReportDiagnostic = z.object({
  kind: z.literal("report-diagnostic"),
});

export type GtbRemoteBackendRequestResponseReportDiagnostic = z.infer<
  typeof GtbRemoteBackendRequestResponseReportDiagnostic
>;

export const GtbRemoteBackendRequestRunFormatter = z.object({
  kind: z.literal("run-formatter"),
  cmd: GtpFormatterCmd,
});

export type GtbRemoteBackendRequestRunFormatter = z.infer<
  typeof GtbRemoteBackendRequestRunFormatter
>;

export const GtbRemoteBackendRequestResponseRunFormatter = z.object({
  kind: z.literal("run-formatter"),
});

export type GtbRemoteBackendRequestResponseRunFormatter = z.infer<
  typeof GtbRemoteBackendRequestResponseRunFormatter
>;

export const GtbRemoteBackendRequestWriteFile = z.object({
  kind: z.literal("write-file"),
  path: z.string(),
  content: z.string(),
});

export type GtbRemoteBackendRequestWriteFile = z.infer<typeof GtbRemoteBackendRequestWriteFile>;

export const GtbRemoteBackendRequest = z.union([
  GtbRemoteBackendRequestGlobFiles,
  GtbRemoteBackendRequestReadFile,
  GtbRemoteBackendRequestFileExists,
  GtbRemoteBackendRequestIsFile,
  GtbRemoteBackendRequestFindFile,
  GtbRemoteBackendRequestReportDiagnostic,
  GtbRemoteBackendRequestRunFormatter,
  GtbRemoteBackendRequestWriteFile,
]);

export type GtbRemoteBackendRequest = z.infer<typeof GtbRemoteBackendRequest>;

export const GtbRemoteBackendRequestResponseWriteFile = z.object({
  kind: z.literal("write-file"),
});

export type GtbRemoteBackendRequestResponseWriteFile = z.infer<
  typeof GtbRemoteBackendRequestResponseWriteFile
>;

export const GtbRemoteBackendRequestResponse = z.union([
  GtbRemoteBackendRequestResponseGlobFiles,
  GtbRemoteBackendRequestResponseReadFile,
  GtbRemoteBackendRequestResponseFileExists,
  GtbRemoteBackendRequestResponseIsFile,
  GtbRemoteBackendRequestResponseFindFile,
  GtbRemoteBackendRequestResponseReportDiagnostic,
  GtbRemoteBackendRequestResponseRunFormatter,
  GtbRemoteBackendRequestResponseWriteFile,
]);

export type GtbRemoteBackendRequestResponse = z.infer<typeof GtbRemoteBackendRequestResponse>;

export const GtbRemoteRuntimeRequestLoadInProject = z.object({
  kind: z.literal("load-in-project"),
});

export type GtbRemoteRuntimeRequestLoadInProject = z.infer<
  typeof GtbRemoteRuntimeRequestLoadInProject
>;

export const GtbRemoteRuntimeRequestResponseLoadInProject = z.object({
  kind: z.literal("load-in-project"),
});

export type GtbRemoteRuntimeRequestResponseLoadInProject = z.infer<
  typeof GtbRemoteRuntimeRequestResponseLoadInProject
>;

export const GtbRemoteRuntimeRequestLoadInModules = z.object({
  kind: z.literal("load-in-modules"),
});

export type GtbRemoteRuntimeRequestLoadInModules = z.infer<
  typeof GtbRemoteRuntimeRequestLoadInModules
>;

export const GtbRemoteRuntimeRequestResponseLoadInModules = z.object({
  kind: z.literal("load-in-modules"),
});

export type GtbRemoteRuntimeRequestResponseLoadInModules = z.infer<
  typeof GtbRemoteRuntimeRequestResponseLoadInModules
>;

export const GtbRemoteRuntimeRequestCompile = z.object({
  kind: z.literal("compile"),
});

export type GtbRemoteRuntimeRequestCompile = z.infer<typeof GtbRemoteRuntimeRequestCompile>;

export const GtbRemoteRuntimeRequest = z.union([
  GtbRemoteRuntimeRequestLoadInProject,
  GtbRemoteRuntimeRequestLoadInModules,
  GtbRemoteRuntimeRequestCompile,
]);

export type GtbRemoteRuntimeRequest = z.infer<typeof GtbRemoteRuntimeRequest>;

export const GtbRemoteRuntimeRequestResponseCompile = z.object({
  kind: z.literal("compile"),
  meta: GtMeta,
});

export type GtbRemoteRuntimeRequestResponseCompile = z.infer<
  typeof GtbRemoteRuntimeRequestResponseCompile
>;

export const GtbRemoteRuntimeRequestResponse = z.union([
  GtbRemoteRuntimeRequestResponseLoadInProject,
  GtbRemoteRuntimeRequestResponseLoadInModules,
  GtbRemoteRuntimeRequestResponseCompile,
]);

export type GtbRemoteRuntimeRequestResponse = z.infer<typeof GtbRemoteRuntimeRequestResponse>;
