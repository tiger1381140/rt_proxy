mod common;
mod config;
mod netio;
mod proxy;

use crate::netio::control::*;
use env_logger;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    env_logger::init();

    // 运行主程序
    let _ = Control::new()
        .ok_or("create control failed")?
        .start_service()
        .await;

    Ok(())
}
