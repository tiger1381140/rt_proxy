use futures::channel::mpsc::Receiver;
use serde_json::Value;
use crate::common::common_file::*;

const LOCAL_JSON_FILE: &str = "/usr/setup/NetworkDLP/config/NDLP/Local.json";

#[derive(Clone)]
pub struct LocalConfigMirror {
    pub _enable: bool,
    pub _interface: String
}

#[derive(Clone)]
pub struct LocalConfigIcapRemote {
    pub _enable: bool,
    pub _ip: String,
    pub _port: u16,
}

#[derive(Clone)]
pub struct LocalJson {
    pub _mirror: LocalConfigMirror,
    pub _icap_remote: LocalConfigIcapRemote,
    pub thread_num: u16
}

impl LocalJson {
    pub fn new() -> Option<Self> {
        let content = match common_open_file(LOCAL_JSON_FILE) {
            Some(c) => c,
            None => return None,
        };

        // 尝试将字符串解析为 JSON
        let json: Value = match serde_json::from_str(&content) {
            Ok(v) => v,
            Err(_) => return None,
        };

        // 从 JSON 中提取各个字段
        let mirror = LocalConfigMirror {
            _enable: json["mirror"]["enable"].as_bool().unwrap_or(false),
            _interface: json["mirror"]["interface"].as_str().unwrap_or("").to_string(),
        };

        let icap_remote = LocalConfigIcapRemote {
            _enable: json["icap-remote"]["enable"].as_bool().unwrap_or(false),
            _ip: json["icap-remote"]["ip"].as_str().unwrap_or("").to_string(),
            _port: json["icap-remote"]["port"].as_u64().unwrap_or(1344) as u16,
        };

        let thread_num = json["icap"]["threadCnt"].as_u64().unwrap_or(1) as u16;

        Some(Self {
            _mirror: mirror,
            _icap_remote: icap_remote,
            thread_num,
        })
    }

    pub fn watch() -> Receiver<()> {
        return common_watch_file(LOCAL_JSON_FILE);
    }
}