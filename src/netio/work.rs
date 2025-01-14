//use notify::Config;
use tokio;
//use tokio::net::{TcpListener, TcpStream};
use crate::config::config_json::ConfigJson;
use crate::config::local_json::LocalJson;
use crate::proxy::http::Http;
use tokio::net::TcpListener;
use tokio::sync::broadcast::Receiver;

pub struct _FiveInfo {
    pub _src_ipv4: u32,
    pub _dst_ipv4: u32,
    pub _src_port: u16,
    pub _dst_port: u16,
    pub _protocol: u8,
}

pub struct Work {
    pub _thread_id: usize,

    pub thread_local_json: Option<LocalJson>,
    pub thread_config_json: Option<ConfigJson>,

    pub thread_http_server: Option<TcpListener>,
}

impl Work {
    pub async fn start_service(
        id: usize,
        mut _local_rx: Receiver<LocalJson>,
        mut config_rx: Receiver<ConfigJson>,
    ) -> Result<(), std::io::Error> {
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
                        work.update_config(message).await;
                    }
                }
                _http_socket = Http::accept_service(&work.thread_http_server) => {
                    if let Ok(_socket) = _http_socket {
                        tokio::spawn(async move {
                            if let Err(e) = Http::process_service(_socket).await {
                                println!("failed to process connection; error = {e}");
                            }
                        });
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

    fn update_local(&mut self, local_json: LocalJson) {
        self.thread_local_json = Some(local_json);
    }

    async fn update_config(&mut self, new_config_json: ConfigJson) {
        if !new_config_json.is_listen_mode() {
            self.thread_config_json = Some(new_config_json);
            self.thread_http_server = None;
            return;
        }
        /* new is listen mode */
        if self.thread_config_json.is_none() {
            let http_listen = TcpListener::bind("0.0.0.0:2128")
                .await
                .expect("Failed to bind to 0.0.0.0:2128");
            self.thread_config_json = Some(new_config_json);
            self.thread_http_server = Some(http_listen);
            return;
        }
        /* self thread_config_json is not none */
        if !self.thread_config_json.as_ref().unwrap().is_listen_mode() {
            let http_listen = TcpListener::bind("0.0.0.0:2128")
                .await
                .expect("Failed to bind to 0.0.0.0:2128");
            self.thread_config_json = Some(new_config_json);
            self.thread_http_server = Some(http_listen);
        }
        /* self thread_config_json is listen mode */
        return;
    }

    pub fn new(id: usize) -> Self {
        return Work {
            _thread_id: id,
            thread_local_json: None,
            thread_config_json: None,
            thread_http_server: None,
        };
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
