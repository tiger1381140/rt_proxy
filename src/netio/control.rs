use futures::channel::mpsc;
use futures::{future, StreamExt};
use tokio::runtime::Runtime;
use tokio::sync::broadcast;

use crate::config::config_json::ConfigJson;
use crate::config::local_json::LocalJson;
use crate::netio::work::Work;

pub struct Control {
    pub local_json: LocalJson,
    pub local_watch_rx: mpsc::Receiver<()>,
    pub local_tx: broadcast::Sender<LocalJson>,
    pub config_json: ConfigJson,
    pub config_watch_rx: mpsc::Receiver<()>,
    pub config_tx: broadcast::Sender<ConfigJson>,
    pub runtimes: Vec<Runtime>,
}

impl Control {
    pub fn new() -> Option<Self> {
        // 获取local_json
        let local_json = match LocalJson::new() {
            Some(json) => json,
            None => return None,
        };
        let local_watch_rx = LocalJson::watch();

        // 获取config_json
        let config_json = match ConfigJson::new() {
            Some(json) => json,
            None => return None,
        };
        let config_watch_rx = ConfigJson::watch();

        // 创建 channel
        let (local_tx, _) = broadcast::channel::<LocalJson>(32);
        let (config_tx, _) = broadcast::channel::<ConfigJson>(32);

        // 创建runtime
        let runtimes = Self::create_runtimes(local_json.thread_num, 1);

        Some(Self {
            local_json: local_json,
            local_watch_rx: local_watch_rx,
            local_tx: local_tx,
            config_json: config_json,
            config_watch_rx: config_watch_rx,
            config_tx: config_tx,
            runtimes: runtimes,
        })
    }

    fn create_runtimes(num: u16, threads_per_runtime: u16) -> Vec<Runtime> {
        (0..num)
            .map(|i| {
                tokio::runtime::Builder::new_multi_thread()
                    .thread_name(format!("runtime-{}", i))
                    .worker_threads(threads_per_runtime as usize)
                    .enable_all()
                    .build()
                    .expect("创建runtime失败")
            })
            .collect()
    }

    pub fn lunch_local_file(&mut self) {
        let local_json = match LocalJson::new() {
            Some(json) => json,
            None => return,
        };
        self.local_json = local_json;
        let _ = self.local_tx.send(self.local_json.clone());
    }

    pub fn lunch_config_file(&mut self) {
        let config_json = match ConfigJson::new() {
            Some(json) => json,
            None => return,
        };
        self.config_json = config_json;
        let _ = self.config_tx.send(self.config_json.clone());
    }

    pub async fn start_service(&mut self) -> Result<u32, String> {
        let mut tasks: Vec<_> = self
            .runtimes
            .iter()
            .enumerate()
            .map(|(i, runtime)| {
                let local_rx = self.local_tx.subscribe();
                let config_rx = self.config_tx.subscribe();
                runtime.spawn(async move {
                    println!("运行时 {} 开始工作", i);
                    let _ = Work::start_service(i, local_rx, config_rx).await;
                })
            })
            .collect();

        let _ = self.local_tx.send(self.local_json.clone());
        let _ = self.config_tx.send(self.config_json.clone());

        while !tasks.is_empty() {
            tokio::select! {
                maybe_local = self.local_watch_rx.next() => {
                    if maybe_local.is_some() {
                        self.lunch_local_file();
                    }
                }
                maybe_config = self.config_watch_rx.next() => {
                    if maybe_config.is_some() {
                        self.lunch_config_file();
                    }
                }
                task_result = future::select_all(&mut tasks) => {
                    let (_, index, _) = task_result;
                    println!("任务 {} 退出", index);
                    tasks.remove(index);
                    if tasks.is_empty() {
                        println!("所有任务已完成，程序退出");
                        break;
                    }
                }
                _ = tokio::signal::ctrl_c() => {
                    println!("接收到中断信号，正在停止所有任务");
                    break;
                }
            }
        }

        Ok(self.runtimes.len() as u32)
    }
}
