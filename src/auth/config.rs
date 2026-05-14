use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

const GLOBAL_CONFIG_FILENAME: &str = ".kaneo-conf.json";
const LOCAL_CONFIG_FILENAME: &str = ".kaneo-conf.json";

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GlobalConfig {
    #[serde(default = "default_instance")]
    pub instance: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub api_key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}

fn default_instance() -> String {
    "cloud.kaneo.app".to_owned()
}

impl GlobalConfig {
    pub fn global_path() -> PathBuf {
        let home = dirs::home_dir().unwrap_or_default();
        home.join(".config").join(GLOBAL_CONFIG_FILENAME)
    }

    pub fn load() -> Result<Self, anyhow::Error> {
        let path = Self::global_path();
        if !path.exists() {
            return Ok(Self::default());
        }
        let data = std::fs::read_to_string(&path)
            .map_err(|e| anyhow::anyhow!("reading {}: {e}", path.display()))?;
        serde_json::from_str(&data).map_err(|e| anyhow::anyhow!("parsing {}: {e}", path.display()))
    }

    pub fn save(&self) -> Result<(), anyhow::Error> {
        let path = Self::global_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| anyhow::anyhow!("creating {}: {e}", parent.display()))?;
        }
        let data = serde_json::to_string_pretty(self)?;
        std::fs::write(&path, data).map_err(|e| anyhow::anyhow!("writing {}: {e}", path.display()))
    }

    pub fn decrypted_api_key(&self) -> Result<Option<String>, crate::errors::KaneoError> {
        match &self.api_key {
            Some(encrypted) => {
                let key = crate::auth::crypto::decrypt(encrypted)?;
                Ok(Some(key))
            }
            None => Ok(None),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LocalConfig {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub workspace_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub project_id: Option<String>,
}

impl LocalConfig {
    pub fn find_from(start: &Path) -> Vec<Self> {
        let mut configs = Vec::new();
        let mut dir = start.to_path_buf();
        let home = dirs::home_dir().unwrap_or_default();

        loop {
            let candidate = dir.join(LOCAL_CONFIG_FILENAME);
            if candidate.is_file()
                && let Ok(data) = std::fs::read_to_string(&candidate)
                && let Ok(cfg) = serde_json::from_str::<LocalConfig>(&data)
            {
                configs.push(cfg);
            }

            if dir == home {
                break;
            }
            if !dir.pop() {
                break;
            }
        }

        configs
    }

    pub fn merge(configs: &[Self]) -> Self {
        let mut merged = Self::default();
        for cfg in configs {
            if merged.workspace_id.is_none() {
                merged.workspace_id.clone_from(&cfg.workspace_id);
            }
            if merged.project_id.is_none() {
                merged.project_id.clone_from(&cfg.project_id);
            }
            if merged.workspace_id.is_some() && merged.project_id.is_some() {
                break;
            }
        }
        merged
    }

    pub fn write_to(dir: &Path, config: &Self) -> Result<(), anyhow::Error> {
        let path = dir.join(LOCAL_CONFIG_FILENAME);
        let data = serde_json::to_string_pretty(config)?;
        std::fs::write(&path, data).map_err(|e| anyhow::anyhow!("writing {}: {e}", path.display()))
    }

    pub fn remove_from(dir: &Path) -> Result<(), anyhow::Error> {
        let path = dir.join(LOCAL_CONFIG_FILENAME);
        if path.exists() {
            std::fs::remove_file(&path)
                .map_err(|e| anyhow::anyhow!("removing {}: {e}", path.display()))?;
        }
        Ok(())
    }
}
