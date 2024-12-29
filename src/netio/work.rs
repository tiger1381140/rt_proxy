//use notify::Config;
use tokio;
//use tokio::net::{TcpListener, TcpStream};
use tokio::net::TcpListener;
use tokio::sync::broadcast::Receiver;
use crate::config::config_json::ConfigJson;
use crate::config::local_json::LocalJson;
use crate::proxy::http::Http;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct _FiveInfo {
    pub _src_ipv4: u32,
    pub _dst_ipv4: u32,
    pub _src_port: u16,
    pub _dst_port: u16,
    pub _protocol: u8
}

pub struct Work {
    pub _thread_id: usize,

    pub thread_local_json: Option<LocalJson>,
    pub thread_config_json: Option<ConfigJson>,

    pub thread_http_server: Option<TcpListener>,
}

impl Work {
    pub async fn start_service(id:usize, mut _local_rx: Receiver<LocalJson>, mut config_rx: Receiver<ConfigJson>) -> Result<(), Box<dyn std::error::Error>> {
        let mut work = Work::new(id);
        loop {
            tokio::select! {
                msg = _local_rx.recv() => {
                    if let Ok(message) = msg {
                        work.update_local(message);
                    }
                }
                msg = config_rx.recv() => {
                    if let Ok(message) = msg {
                        work.update_config(message);
                    }
                }
                _http_socket = Http::accept_service(&work.thread_http_server) => {
                    if let Ok(_socket) = _http_socket {
                        println!("接收到中断信号，正在停止所有任务");
                    }
                }
                _ = tokio::signal::ctrl_c() => {
                    println!("接收到中断信号，正在停止所有任务");
                    break;
                }
            }
        }
        Ok(())
    }

    pub fn update_config(&mut self, config_json: ConfigJson) {
        if self.thread_config_json.is_none() {

            self.thread_config_json = Some(config_json);
        }
    }
    pub fn update_local(&mut self, local_json: LocalJson) {
        self.thread_local_json = Some(local_json);
    }


    pub fn new(id:usize) -> Self {
        return Work {
            _thread_id: id,
            thread_local_json: None,
            thread_config_json: None,

            thread_http_server: None
        }
    }

    fn bind_service(&mut self, _config_json: &ConfigJson) {
        //self.thread_http_server = Some(TcpListener::bind(&config_json._http_listen).await?);
    }

    async fn accept_service(&mut self) -> Result<_FiveInfo, Box<dyn std::error::Error>> {
        if self.thread_config_json.is_none() {
            return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "配置 JSON 为空")));
        }
        //key = accetp().await
        let a = _FiveInfo {
            _src_ipv4: 12,
            _dst_ipv4: 13,
            _src_port: 14,
            _dst_port: 15,
            _protocol: 8
        };
        Ok(a)
    }
}

/* 
impl Clone for Work {
    fn clone(&self) -> Self {
        Work {
            _thread_id: self._thread_id,
            thread_local_rx: Arc::clone(&self.thread_local_rx),
            thread_local_json: self.thread_local_json.clone(),
            thread_config_rx: Arc::clone(&self.thread_config_rx),
            thread_config_json: self.thread_config_json.clone(),
        }
    }
}
*/