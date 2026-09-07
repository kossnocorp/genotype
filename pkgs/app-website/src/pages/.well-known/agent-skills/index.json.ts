import type { APIRoute } from "astro";
import { SkillHandlers } from "../../../skill/SkillHandlers.ts";

export const prerender = false;

export const GET: APIRoute = () => SkillHandlers.index();
