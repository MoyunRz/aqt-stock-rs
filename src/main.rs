use aqt_stock::config::config;
use aqt_stock::tasks;
use tasks::sty;
use std::error::Error;
use log::{ error, debug };
use log4rs;
use dotenv::dotenv;
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // 加载 .env 文件中的配置
    dotenv().ok();
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    let cfg = config::Configs::load().expect("TODO: panic message");
    debug!("启动配置 {:?}", cfg);
    sty::start_sty(cfg).await.unwrap_or_else(|e|{
        error!("策略启动出错 {}", e);
    });
    Ok(())
}
