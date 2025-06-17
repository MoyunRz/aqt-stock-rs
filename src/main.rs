use aqt_stock::config::config;
use aqt_stock::tasks;
use tasks::sty;
use std::error::Error;
use log::{info,error};
use log4rs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let cfg = config::Configs::load().expect("TODO: panic message");
    info!("启动配置 {:?}", cfg);
    sty::start_sty(cfg).await.unwrap_or_else(|e|{
        error!("策略启动出错 {}", e);
    });
    Ok(())
}
