use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum TargetPath{
    Disk,
    ProgramFolder,
    UserFolder,
    Custom(String)
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Mode {
    Ban,
    Access,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SecurityRule {
    pub path: TargetPath,
    pub mode: Mode
}