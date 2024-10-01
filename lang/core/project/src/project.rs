use genotype_project::project::GTProject;

pub trait GTProjectOut {
    fn generate(project: &GTProject, out: &str) -> Result<Self, Box<dyn std::error::Error>>
    where
        Self: Sized;
}
