/// <reference types="../.astro/types.d.ts" />

// TODO: Enable lines before after this PR is merged: https://github.com/withastro/starlight/pull/3572

// NOTE: Currently, the problem is fixed by 'src/env.astro' file which
// references are included in all Astro files just because it is included in
// 'tsconfig.json' file.

// /// <reference types="../node_modules/@astrojs/starlight/virtual.d.ts" />
// /// <reference types="../node_modules/@astrojs/starlight/virtual-internal.d.ts" />

// declare namespace App {
//   type StarlightLocals = import("@astrojs/starlight").StarlightLocals;
//   interface Locals extends StarlightLocals {}
// }
