# Working On a New Target

## Target Project

### Traits to Implement

The starting point for implementing a new target project is `GtlProject`. It defines several supertrait and associated types with trait bounds that create chain reaction leading to a complete implementation of the target project.

Here's the `GtlProject` traits hierarchy chart:

```mermaid
graph TD
    %% region: GtlProject ------------------------------------------------------

    GtlProject{{GtlProject}}:::trait

    %% Supertraits

    GtlProject -- Self::Module --> GtlProjectManifestGenerator

    %% Associated Types

    GtlProject -- type --> GtlProjectTypeModule[Self::Module]:::type
    GtlProjectTypeModule -- bound --> GtlModule

    GtlProject -- type --> GtlProjectTypeLangConfig[Self::LangConfig]:::type
    GtlProjectTypeLangConfig -- bound --> GtlConfig

    %% endregion

    %% region: GtlProjectManifestGenerator -------------------------------------------

    GtlProjectManifestGenerator{{GtlProjectManifestGenerator}}:::trait

    %% Generics

    GtlProjectManifestGenerator -- generic --> GtlModule

    %% Associated Types

    GtlProjectManifestGenerator -- type --> GtlProjectManifestGeneratorTypeManifestDependency[Self::ManifestDependency]:::type
    GtlProjectManifestGeneratorTypeManifestDependency -- bound --> GtlProjectManifestDependency

    GtlProjectManifestGenerator -- type --> GtlProjectManifestGeneratorTypeLangConfig[Self::LangConfig]:::type
    GtlProjectManifestGeneratorTypeLangConfig -- bound --> GtlConfig

    %% endregion

    %% region: GtlProjectManifestDependency ------------------------------------------

    GtlProjectManifestDependency{{GtlProjectManifestDependency}}:::trait

    %% Associated Types

    GtlProjectManifestDependency -- type --> GtlProjectManifestDependencyTypeDependencyIdent[Self::DependencyIdent]:::type
    GtlProjectManifestDependencyTypeDependencyIdent -- bound --> GtlDependencyIdent

    %% endregion

    %% region: GtlModule -------------------------------------------------------

    GtlModule{{GtlModule}}:::trait

    %% Associated Types

    GtlModule -- type --> GtlModuleTypeImport[Self::Import]:::type
    GtlModuleTypeImport -- bound --> GtlImport

    %% endregion

    %% region: GtlImport -------------------------------------------------------

    GtlImport{{GtlImport}}:::trait

    %% Associated Types

    GtlImport -- type --> GtlImportTypeDependencyIdent[Self::DependencyIdent]:::type
    GtlImportTypeDependencyIdent -- bound --> GtlDependencyIdent

    GtlImport -- type --> GtlImportTypeImportRef[Self::ImportRef]:::type
    GtlImportTypeImportRef -- bound --> GtlImportRef

    GtlImport -- type --> GtlImportTypeImportRefName[Self::ImportRefName]:::type
    GtlImportTypeImportRefName -- bound --> GtlImportRefName

    %% endregion

    %% region: GtlConfig -------------------------------------------------------

    GtlConfig{{GtlConfig}}:::trait

    %% Supertraits

    GtlConfig -- supertrait --> GtlConfigHealth

    %% endregion

    %% region: GtlConfigHealth -------------------------------------------------

    GtlConfigHealth{{GtlConfigHealth}}:::trait

    %% endregion

    %% region: GtlDependencyIdent ----------------------------------------------

    GtlDependencyIdent{{GtlDependencyIdent}}:::trait

    %% Associated Types

    GtlDependencyIdent -- type --> GtlPath[Self::Path]:::type

    %% endregion

    %% region: GtlImportRef ----------------------------------------------------

    GtlImportRef{{GtlImportRef}}:::trait

    %% endregion

    %% region: GtlImportRefName ------------------------------------------------

    GtlImportRefName{{GtlImportRefName}}:::trait

    %% endregion

    %% region: GtlPath ------------------------------------------------

    GtlPath{{GtlPath}}:::trait

    %% endregion

    classDef type opacity:0.75,stroke:#140977
    classDef trait opacity:1,stroke:#B100B4,stroke-width:2px
```
