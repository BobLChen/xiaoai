use tokio::time::sleep;
use tokio_tungstenite::{connect_async};
use xiaoai::base::AppError;
use xiaoai::services::message::{MessageManager, WsStream};
use xiaoai::services::monitor::InstructionMonitor;
use std::time::Duration;

async fn connect(url: &str) -> Result<WsStream, AppError> {
    let (ws_stream, _) = connect_async(url).await?;
    Ok(WsStream::Client(ws_stream))
}

#[tokio::main]
async fn main() {

    let url = std::env::args().nth(1).expect("❌ 请输入服务器地址");
    println!("✅ 已启动");
    
    loop {
        println!("🔗 尝试链接: {}", url);
        let Ok(ws_stream) = connect(&url).await else {
            sleep(Duration::from_secs(1)).await;
            continue;
        };
        println!("✅ 已连接: {:?}", url);

        MessageManager::instance().init(ws_stream).await;

        InstructionMonitor::start(|data| async move {
            MessageManager::instance().send_event(data).await
        }).await;

        if let Err(e) = MessageManager::instance().process_messages().await {
            eprintln!("❌ 消息处理异常: {}", e);
        }

        MessageManager::instance().dispose().await;
    }
}

