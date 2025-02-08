# Genotype immediate TODOs

Currently I'm working on the workspaces feature. It will enable LSP and CLI watch mode.

At the moment the goal is to move modules parsing from the the project to the workspace crate and rework CLI to use it. It will allow building multiple projects from the same set of modules which is crucial for LSP.

- [x] Figure out approach to [`source_code`](./parser/src/module/mod.rs:21) field in the `GTModule`.

  It contains the path which must be relative to the currently active working directory to make it identify properly in editors, i.e. when using VS Code workspace feature. And since modules can be shared between projects, the path must differ depending on the context.

  Another issue with it is that the source code is stored in the workspace and copying it to the module is a waste of memory.

  The source code is needed to format parsing errors and it is easy to provide it when needed, rather than storing it in the module.

- [ ] Figure out approach to [`id`](./parser/src/module/mod.rs:13) field in the `GTModule`.

  Currently the id is a module path without extension relative to the project. Since workspaces can contain multiple projects and share the same modules between them, it is better to make module ids relative to the workspace root.

  A curve ball is that editors can open files from outside of the workspace and the module id should probably be an absolute path.

  At the moment the module creates id from the string when parsing. It is better to move it outiside.

- [ ] Move module parsing & error reporting from the project to workspace crate so that parsed modules can be shared between projects (e.g. as external modules), one can open orphan modules or modules ouside the workspace.

  Projects should be responsible for building and linking modules.

- [ ] Find a better place for `source_code` in a project.

  Right now I have to store it in `GTModuleParse` and `GTProjectModule`, so that it is available for error reporting, however, it should come from a single place to save memory.

  The problem is that `NamedSource` contains the path which must be relative to the currently active working directory to make it identify properly in editors, i.e. when using VS Code workspace feature. And since modules can be shared between projects, the path must differ depending on the context.

  The right place for it is the project crate, but the source code itself must come as a string reference, i.e. from the workspace crate.
