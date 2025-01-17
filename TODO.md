# Genotype immediate TODOs

Currently I'm working on the workspaces feature. It will enable LSP and CLI watch mode.

At the moment the goal is to move modules parsing from the the project to the workspace crate and rework CLI to use it. It will allow building multiple projects from the same set of modules which is crucial for LSP.

- [ ] Figure out approach to [`source_code`](./parser/src/module/mod.rs:23) field in the `GTModule`.

  It contains the path which must be relative to the currently active working directory to make it identify properly in editors, i.e. when using VS Code workspace feature. And since modules can be shared between projects, the path must differ depending on the context.

  Another issue with it is that the source code is stored in the workspace and copying it to the module is a waste of memory.

  The source code is needed to format parsing errors and it is easy to provide it when needed, rather than storing it in the module.

- [ ] Figure out approach to [`id`](./parser/src/module/mod.rs:13) field in the `GTModule`.

  Currently the id is a module path without extension relative to the project. Since workspaces can contain multiple projects and share the same modules between them, it is better to make module ids relative to the workspace root.

  A curve ball is that editors can open files from outside of the workspace and the module id should probably be an absolute path.

  At the moment the module creates id from the string when parsing. It is better to move it outiside.
