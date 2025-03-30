use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct IncBuildDependency
{
    pub file: String,
    pub hash: u64,
}

impl PartialEq for IncBuildDependency
{
    fn eq(&self, other: &Self) -> bool {
        self.file == other.file && self.hash == other.hash
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct IncBuildFile
{
    pub file: String,
    pub hash: u64,
    pub dependencies: Vec<IncBuildDependency>,
}
