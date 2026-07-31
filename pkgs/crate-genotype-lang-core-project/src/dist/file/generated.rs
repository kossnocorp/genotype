use crate::prelude::internal::*;

#[derive(Debug, PartialEq, Clone, Serialize)]
pub struct GtlDistFileGenerated {
    pub path: GtpTargetFilePath,
    pub source_code: GtpSourceCode,
    pub source_module_id: Option<GtModuleId>,
}

impl GtlDistFileGenerated {
    pub fn as_build_info_dist_file(&self) -> GtpBuildInfoDistFile {
        GtpBuildInfoDistFile {
            hash: self.source_code.hash.clone(),
            src_id: self.source_module_id.clone(),
        }
    }
}
