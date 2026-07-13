import type { AstroComponentFactory as Comp } from "astro/runtime/server/index.js";

export namespace IconTypes {
  export type Props = PropsDirect | PropsProp;

  export type PropsDirect<Extra = {}> = (PropsId | PropsComponent) & Extra;

  export type Prop<Extra = {}> = string | Comp | PropsDirect<Extra>;

  export interface PropsProp {
    prop: Prop;
  }

  export interface PropsComponent extends PropsBase {
    Factory: Comp;
  }

  export interface PropsId extends PropsBase {
    id: string;
  }

  export interface PropsBase {
    class?: string;
    style?: Style;
  }

  export type Style = "100" | "200" | "300" | "brands" | "dev" | "assets";
}
