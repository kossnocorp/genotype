use crate::prelude::internal::*;

pub type GtlGenerations<ProjectModule> = (Vec<GtlGeneration<ProjectModule>>, Option<Vec<GtNotice>>);

pub struct GtlGeneration<ProjectModule: GtlProjectModule> {
    pub file: GtlProjectFile<ProjectModule>,
    pub notices: Option<Vec<GtNotice>>,
}

impl<ProjectModule: GtlProjectModule> GtlGeneration<ProjectModule> {
    pub fn file<File: Into<GtlProjectFile<ProjectModule>>>(file: File) -> Self {
        GtlGeneration {
            file: file.into(),
            notices: None,
        }
    }

    pub fn file_with_notice<File: Into<GtlProjectFile<ProjectModule>>>(
        file: File,
        notice: GtNotice,
    ) -> Self {
        Self::file_with_notices(file, vec![notice])
    }

    pub fn file_with_notice_option<File: Into<GtlProjectFile<ProjectModule>>>(
        file: File,
        notice: Option<GtNotice>,
    ) -> Self {
        match notice {
            Some(notice) => Self::file_with_notices(file, vec![notice]),
            None => Self::file(file),
        }
    }

    pub fn file_with_notices<File: Into<GtlProjectFile<ProjectModule>>>(
        file: File,
        notices: Vec<GtNotice>,
    ) -> Self {
        GtlGeneration {
            file: file.into(),
            notices: Some(notices),
        }
    }
}

impl<ProjectModule: GtlProjectModule> From<(GtlProjectFile<ProjectModule>, Vec<GtNotice>)>
    for GtlGeneration<ProjectModule>
{
    fn from((file, notices): (GtlProjectFile<ProjectModule>, Vec<GtNotice>)) -> Self {
        Self::file_with_notices(file, notices)
    }
}

impl<ProjectModule: GtlProjectModule> From<GtlProjectFile<ProjectModule>>
    for GtlGeneration<ProjectModule>
{
    fn from(file: GtlProjectFile<ProjectModule>) -> Self {
        Self::file(file)
    }
}

impl<ProjectModule: GtlProjectModule> From<(GtlProjectFile<ProjectModule>, GtNotice)>
    for GtlGeneration<ProjectModule>
{
    fn from((file, notice): (GtlProjectFile<ProjectModule>, GtNotice)) -> Self {
        Self::file_with_notice(file, notice)
    }
}

impl<ProjectModule: GtlProjectModule> From<(GtlProjectFile<ProjectModule>, Option<GtNotice>)>
    for GtlGeneration<ProjectModule>
{
    fn from((file, notice): (GtlProjectFile<ProjectModule>, Option<GtNotice>)) -> Self {
        Self::file_with_notice_option(file, notice)
    }
}

// pub struct GtlCompilerGenerationFiles<Module: GtlModule> {
//     pub files: Vec<GtlProjectFile<Module>>,
//     pub notices: Vec<GtNotice>,
// }

// impl<Module: GtlModule> GtlCompilerGenerationFiles<Module> {
//     pub fn files(files: Vec<GtlProjectFile<Module>>) -> Self {
//         GtlCompilerGenerationFiles {
//             files,
//             notices: vec![],
//         }
//     }

//     pub fn new(files: Vec<GtlProjectFile<Module>>, notices: Vec<GtNotice>) -> Self {
//         GtlCompilerGenerationFiles { files, notices }
//     }
// }

// impl<Module: GtlModule> From<Vec<GtlProjectFile<Module>>> for GtlCompilerGenerationFiles<Module>
// where
//     Module: GtlModule,
// {
//     fn from(files: Vec<GtlProjectFile<Module>>) -> Self {
//         GtlCompilerGenerationFiles::files(files)
//     }
// }
