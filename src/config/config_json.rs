use futures::channel::mpsc::Receiver;
use serde_json;
use crate::common::common_file::*;
use serde::{Deserialize, Serialize};

const CONFIG_JSON_FILE: &str = "/usr/setup/NetworkDLP/config/NDLP/NdlpConfig.json";

#[derive(Serialize, Deserialize, Clone)]
#[serde(default, rename_all = "PascalCase")]
pub struct ConfigJson {
    pub client_mode: String,
}

impl ConfigJson {
    pub fn new() -> Option<Self> {
        let content= match common_open_file(CONFIG_JSON_FILE) {
            Some(c) => c,
            None => return None,
        };

        let config_json: ConfigJson = serde_json::from_str(&content).expect("Failed to parse JSON");
        return Some(config_json);
    }

    pub fn watch() -> Receiver<()> {
        return common_watch_file(CONFIG_JSON_FILE);
    }
    
    pub fn get_client_mode(&self) -> String {
        return self.client_mode.clone();
    }
}

impl Default for ConfigJson {  // 因为使用了 serde(default)，需要实现 Default
    fn default() -> Self {
        Self {
            client_mode: String::new(),
        }
    }
}