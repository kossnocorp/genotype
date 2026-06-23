# Working On a New Target

## Target Project

### Traits to Implement

The starting point for implementing a new target project is `GtlCompiler`. It defines the target compiler, project module, and manifest types that create a chain reaction leading to a complete implementation of the target project.

Here's the `GtlCompiler` component hierarchy chart:

```mermaid
graph TD
    %% region: Compiler --------------------------------------------------------

    GtlCompiler{{GtlCompiler}}:::trait
    NewCompiler[NewCompiler]:::newType

    NewCompiler -- implements --> GtlCompiler
    GtlCompiler -- Self::ProjectModule --> NewProjectModule
    GtlCompiler -- Self::Manifest --> NewManifest
    GtlCompiler -- uses --> GtlConfig

    %% endregion

    %% region: Project Module --------------------------------------------------

    GtlProjectModule{{GtlProjectModule}}:::trait
    NewProjectModule[NewProjectModule]:::newType

    NewProjectModule -- implements --> GtlProjectModule
    GtlProjectModule -- Self::LangConfig --> NewConfig
    GtlProjectModule -- Self::Module --> NewModule
    GtlProjectModule -- optional --> ResolveModules[resolve_modules]:::type

    %% endregion

    %% region: Manifest --------------------------------------------------------

    GtlManifest{{GtlManifest}}:::trait
    NewManifest[NewManifest]:::newType

    NewManifest -- implements --> GtlManifest
    GtlManifest -- Self::ProjectModule --> NewProjectModule
    GtlManifest -- reads --> GtlDependencyIdent

    %% endregion

    %% region: Module ----------------------------------------------------------

    GtlModule{{GtlModule}}:::trait
    NewModule[NewModule]:::newType

    NewModule -- implements --> GtlModule
    GtlModule -- supertrait --> GtlRender
    GtlModule -- Self::Import --> NewImport
    GtlModule -- Self::RenderTypes --> NewRenderTypes

    %% endregion

    %% region: Import ----------------------------------------------------------

    GtlImport{{GtlImport}}:::trait
    NewImport[NewImport]:::newType

    NewImport -- implements --> GtlImport
    GtlImport -- Self::DependencyIdent --> NewDependencyIdent
    GtlImport -- Self::ImportRef --> NewImportRef
    GtlImport -- Self::ImportRefName --> NewImportRefName

    %% endregion

    %% region: Render ----------------------------------------------------------

    GtlRender{{GtlRender}}:::trait
    GtlRenderTypes{{GtlRenderTypes}}:::trait
    NewRenderTypes[NewRenderTypes]:::newType

    NewRenderTypes -- implements --> GtlRenderTypes
    GtlRender -- generic --> GtlRenderTypes
    GtlRenderTypes -- Self::State --> NewRenderState
    GtlRenderTypes -- Self::Context --> NewRenderContext
    GtlRenderTypes -- Self::Error --> NewRenderError

    %% endregion

    %% region: Supporting Traits -----------------------------------------------

    GtlConfig[GtlConfig]:::type
    GtpLangConfig{{GtpLangConfig}}:::trait
    GtlDependencyIdent{{GtlDependencyIdent}}:::trait
    GtlImportRef{{GtlImportRef}}:::trait
    GtlImportRefName{{GtlImportRefName}}:::trait
    GtlRenderState{{GtlRenderState}}:::trait
    GtlRenderContext{{GtlRenderContext}}:::trait
    GtlError{{GtlError}}:::trait

    NewConfig[NewConfig]:::newType
    NewDependencyIdent[NewDependencyIdent]:::newType
    NewImportRef[NewImportRef]:::newType
    NewImportRefName[NewImportRefName]:::newType
    NewRenderState[NewRenderState]:::newType
    NewRenderContext[NewRenderContext]:::newType
    NewRenderError[NewRenderError]:::newType

    GtlConfig -- contains --> GtpLangConfig
    NewConfig -- implements --> GtpLangConfig
    NewDependencyIdent -- implements --> GtlDependencyIdent
    NewImportRef -- implements --> GtlImportRef
    NewImportRefName -- implements --> GtlImportRefName
    NewRenderState -- implements --> GtlRenderState
    NewRenderContext -- implements --> GtlRenderContext
    NewRenderError -- implements --> GtlError

    %% endregion

    classDef type opacity:0.75,stroke:#140977
    classDef newType opacity:1,stroke:lime,stroke-width:2px
    classDef trait opacity:1,stroke:#B100B4,stroke-width:2px
```
