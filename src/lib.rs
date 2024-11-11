use std::path::PathBuf;

use security::SecurityRule;
use serde::{Deserialize, Serialize};

pub mod security;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigKill {
    pub cred_list: Vec<security::SecurityRule>
}

impl Default for ConfigKill {
    fn default() -> Self {
        Self {
            cred_list: vec![SecurityRule {mode: security::Mode::Ban, path: security::TargetPath::Disk}]
        }
    }
}

impl ConfigKill {
    fn verificate_path(&self, path: &PathBuf) -> std::io::Result<bool> {
        todo!()
    }
}