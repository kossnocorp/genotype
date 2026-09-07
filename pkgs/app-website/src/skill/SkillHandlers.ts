import { Skill } from "@genotype-lang/skill";
import z from "zod";
import { env } from "cloudflare:workers";

export abstract class SkillHandlers {
  static UpstreamError = class UpstreamError extends Error {
    status: number;

    constructor(status: number) {
      super("Genotype skill release is unavailable");
      this.status = status;
    }
  };

  static #cacheHeaders = {
    "Cache-Control": "public, max-age=300, s-maxage=300",
    "Access-Control-Allow-Origin": "*",
  };

  static async index(): Promise<Response> {
    try {
      const { release, manifest } = await this.#getLatestSkill();

      const skill = {
        name: manifest.name,
        description: manifest.description,
        digest: manifest.digest,
        type: "archive",
        url: `./genotype.zip?tag=${encodeURIComponent(release.tag_name)}`,
      };

      const discovery = {
        $schema: "https://schemas.agentskills.io/discovery/0.2.0/schema.json",
        skills: [skill],
      };

      return Response.json(discovery, { headers: this.#cacheHeaders });
    } catch (err) {
      return this.#errResponse(err);
    }
  }

  static #tagPattern = /^v\d+\.\d+\.\d+(?:[-+][\w.-]+)?$/;

  static async archive(url: URL): Promise<Response> {
    const tag = url.searchParams.get("tag") ?? undefined;
    if (tag !== undefined && !this.#tagPattern.test(tag))
      return new Response("Invalid release tag\n", { status: 400 });

    try {
      const release = await this.#getArchiveRelease(tag);
      const assetUrl = this.#releaseAssetUrl(release, "genotype-skill.zip");
      const assetResp = await this.#fetchGhApi(assetUrl, "application/octet-stream");

      return new Response(assetResp.body, {
        headers: {
          ...this.#cacheHeaders,
          "Content-Type": "application/zip",
          "Content-Disposition": `attachment; filename="genotype-skill-${release.tag_name}.zip"`,
        },
      });
    } catch (err) {
      return this.#errResponse(err);
    }
  }

  static async #getArchiveRelease(tag: string | undefined) {
    if (tag) {
      return this.#getRelease(tag);
    } else {
      const skill = await this.#getLatestSkill();
      return skill.release;
    }
  }

  static #repoUrl = "https://api.github.com/repos/kossnocorp/genotype";

  static async #getRelease(tag?: string): Promise<GhApi.Release> {
    const tagPath = tag ? `tags/${encodeURIComponent(tag)}` : "latest";
    const releaseUrl = `${this.#repoUrl}/releases/${tagPath}`;
    const releaseResp = await this.#fetchGhApi(releaseUrl);
    const releaseRaw = await releaseResp.json();
    const releaseParse = GhApi.Release.safeParse(releaseRaw);

    if (!releaseParse.success) throw new SkillHandlers.UpstreamError(502);
    const release = releaseParse.data;

    return release;
  }

  static #latestSkill: SkillHandlers.Skill | undefined;

  static async #getLatestSkill() {
    if (this.#latestSkill && this.#latestSkill.expires > Date.now()) return this.#latestSkill.value;

    const release = await this.#getRelease();
    this.#releaseAssetUrl(release, "genotype-skill.zip");

    const assetUrl = this.#releaseAssetUrl(release, "genotype-skill.json");
    const assetResp = await this.#fetchGhApi(assetUrl);

    const manifestRaw = await assetResp.json();
    const manifestParse = Skill.Manifest.safeParse(manifestRaw);

    if (!manifestParse.success) throw new SkillHandlers.UpstreamError(502);
    const manifest = manifestParse.data;

    const value = { release, manifest };

    this.#latestSkill = { expires: Date.now() + 300_000, value };

    return value;
  }

  static async #fetchGhApi(
    url: string,
    acceptHeader: string = "application/vnd.github+json",
  ): Promise<Response> {
    const response = await fetch(url, {
      headers: {
        Accept: acceptHeader,
        "User-Agent": "https://genotype-lang.org",
        "X-GitHub-Api-Version": "2022-11-28",
        Authorization: `Bearer ${env.GITHUB_TOKEN}`,
      },
      signal: AbortSignal.timeout(15_000),
    });

    if (!response.ok) throw new SkillHandlers.UpstreamError(response.status === 404 ? 404 : 503);
    return response;
  }

  static #releaseAssetUrl(release: GhApi.Release, name: string): string {
    const asset = release.assets.find((asset) => asset.name === name);
    if (!asset) throw new SkillHandlers.UpstreamError(404);
    return asset.browser_download_url;
  }

  static #errResponse(error: unknown): Response {
    const status = error instanceof this.UpstreamError ? error.status : 503;
    return new Response("Genotype skill release is unavailable. Try again later.", {
      status,
      headers: { "Cache-Control": "no-store", "Retry-After": "60" },
    });
  }
}

export namespace SkillHandlers {
  export interface Skill {
    expires: number;
    value: SkillValue;
  }

  export interface SkillValue {
    release: GhApi.Release;
    manifest: Skill.Manifest;
  }
}

abstract class GhApi {
  static Asset = z.object({
    name: z.string(),
    browser_download_url: z.string(),
  });

  static Release = z.object({
    tag_name: z.string(),
    draft: z.boolean(),
    prerelease: z.boolean(),
    assets: z.array(this.Asset),
  });
}

namespace GhApi {
  export type Asset = z.infer<typeof GhApi.Asset>;

  export type Release = z.infer<typeof GhApi.Release>;
}
