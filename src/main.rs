mod common;
mod netio;
mod config;
mod proxy;

use tokio;
use env_logger;
use crate::netio::control::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    // 初始化日志
    env_logger::init();

    // 运行主程序
    let _= Control::new().ok_or("create control failed")?.start_service().await;

    Ok(())
}
