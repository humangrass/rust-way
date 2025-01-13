use anyhow::anyhow;
use multitool_hg::database::config::DatabaseConfig;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub host: String,
    pub port: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BartenderConfig {
    pub database: DatabaseConfig,
    pub app: AppConfig,
}

impl BartenderConfig {
    pub(crate) fn new(file_path: &Path) -> anyhow::Result<Self> {
        let mut file = File::open(file_path)
            .map_err(|err| anyhow!("Can't open file {:?}: {}", file_path, err))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|err| anyhow!("Can't read {:?}: {}", file_path, err))?;
        let config = serde_yaml::from_str(&contents)
            .map_err(|err| anyhow!("Can't read yaml {:?}: {}", file_path, err))?;
        Ok(config)
    }
}
