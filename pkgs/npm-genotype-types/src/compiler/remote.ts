import { z } from "zod";
import { GtcMetaLoadedProject, GtcMetaLoadedModules, GtcMetaCompiled } from "./meta.js";

export const GtcRemoteRuntimeRequestLoadInProject = z.object({
  kind: z.literal("load-in-project"),
});

export type GtcRemoteRuntimeRequestLoadInProject = z.infer<
  typeof GtcRemoteRuntimeRequestLoadInProject
>;

export const GtcRemoteRuntimeRequestResponseLoadInProject = z.object({
  kind: z.literal("load-in-project"),
  meta: GtcMetaLoadedProject,
});

export type GtcRemoteRuntimeRequestResponseLoadInProject = z.infer<
  typeof GtcRemoteRuntimeRequestResponseLoadInProject
>;

export const GtcRemoteRuntimeRequestLoadInModules = z.object({
  kind: z.literal("load-in-modules"),
});

export type GtcRemoteRuntimeRequestLoadInModules = z.infer<
  typeof GtcRemoteRuntimeRequestLoadInModules
>;

export const GtcRemoteRuntimeRequestResponseLoadInModules = z.object({
  kind: z.literal("load-in-modules"),
  meta: GtcMetaLoadedModules,
});

export type GtcRemoteRuntimeRequestResponseLoadInModules = z.infer<
  typeof GtcRemoteRuntimeRequestResponseLoadInModules
>;

export const GtcRemoteRuntimeRequestCompile = z.object({
  kind: z.literal("compile"),
});

export type GtcRemoteRuntimeRequestCompile = z.infer<typeof GtcRemoteRuntimeRequestCompile>;

export const GtcRemoteRuntimeRequest = z.union([
  GtcRemoteRuntimeRequestLoadInProject,
  GtcRemoteRuntimeRequestLoadInModules,
  GtcRemoteRuntimeRequestCompile,
]);

export type GtcRemoteRuntimeRequest = z.infer<typeof GtcRemoteRuntimeRequest>;

export const GtcRemoteRuntimeRequestResponseCompile = z.object({
  kind: z.literal("compile"),
  meta: GtcMetaCompiled,
});

export type GtcRemoteRuntimeRequestResponseCompile = z.infer<
  typeof GtcRemoteRuntimeRequestResponseCompile
>;

export const GtcRemoteRuntimeRequestResponse = z.union([
  GtcRemoteRuntimeRequestResponseLoadInProject,
  GtcRemoteRuntimeRequestResponseLoadInModules,
  GtcRemoteRuntimeRequestResponseCompile,
]);

export type GtcRemoteRuntimeRequestResponse = z.infer<typeof GtcRemoteRuntimeRequestResponse>;
