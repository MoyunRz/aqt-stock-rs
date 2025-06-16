use aqt_stock::config::config;
use aqt_stock::tasks;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cfg = config::Configs::load().expect("TODO: panic message");
    tasks::sty::start_strategy(cfg).await.unwrap_or_else(|e|{
        panic!("策略启动出错 {}", e);
    });
    Ok(())
}
