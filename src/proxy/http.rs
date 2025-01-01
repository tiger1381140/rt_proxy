//use notify::Config;
use tokio;
//use tokio::net::{TcpListener, TcpStream};
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::sync::broadcast::Receiver;
use crate::config::{config_json::ConfigJson, local_json::LocalJson};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct _FiveInfo {
    pub _src_ipv4: u32,
    pub _dst_ipv4: u32,
    pub _src_port: u16,
    pub _dst_port: u16,
    pub _protocol: u8
}

pub struct Http {
    pub _thread_id: usize,

    pub thread_local_rx: Arc<Mutex<Receiver<LocalJson>>>,
    pub thread_local_json: Option<LocalJson>,

    pub thread_config_rx: Arc<Mutex<Receiver<ConfigJson>>>,
    pub thread_config_json: Option<ConfigJson>,

    pub _thread_http_server: Option<TcpListener>,
    pub _thread_https_server: Option<TcpListener>,

    pub _thread_smtp_server: Option<TcpListener>,
    pub _thread_smtps_server: Option<TcpListener>,

    pub _thread_pop3_server: Option<TcpListener>,
    pub _thread_pop3s_server: Option<TcpListener>,

    pub _thread_imap_server: Option<TcpListener>,
    pub _thread_imaps_server: Option<TcpListener>,

    pub _thread_ftp_server: Option<TcpListener>,
    pub _thread_ftps_server: Option<TcpListener>,

    pub _thread_ftp_data_server: Option<TcpListener>,
    pub _thread_ftps_data_server: Option<TcpListener>,

    pub _thread_ssh_server: Option<TcpListener>,
}

impl Http {
    pub async fn accept_service(http_listen: &Option<TcpListener>) -> Result<TcpStream, std::io::Error> {
        if http_listen.is_none() {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "HTTP 监听器为空"));
        }
        let listener = http_listen.as_ref().unwrap();
        let (mut socket, _) = listener.accept().await?;
        Ok(socket)
    }

    /* 
    fn bind_service(&mut self) {

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

    pub async fn start_service(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let mut local_rx = self.thread_local_rx.lock().await;
        let mut config_rx = self.thread_config_rx.lock().await;
        //let mut local_rx = local_rx.clone();
        loop {
            tokio::select! {
                msg = local_rx.recv() => {
                    if let Ok(_message) = msg {
                        self.thread_local_json = Some(_message);
                    }
                }
                msg = config_rx.recv() => {
                    if let Ok(_message) = msg {
                        let new_config_json = _message;
                        self.thread_config_json = Some(new_config_json );
                        self.bind_service(&new_config_json);
                        self.thread_local_json = None;
                    }
                }
                /* 
                if let Some(http_listen) = &self._thread_http_server => {

                } else {

                }*/
                /* 
                _msg = self.accept_service() => match _msg {
                    Ok(_key) => { self.thread_local_json = None; }
                    Err(_) => { }
                },*/
                _ = tokio::signal::ctrl_c() => {
                    println!("接收到中断信号，正在停止所有任务");
                    break;
                }
            }
        }
        Ok(())
    }
    */

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