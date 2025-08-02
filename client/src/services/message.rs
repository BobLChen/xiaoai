use futures::stream::{SplitSink, SplitStream};
use futures::{SinkExt, StreamExt};
use tokio::process::Command;
use std::sync::{Arc, LazyLock};
use tokio::net::TcpStream;
use tokio::sync::{Mutex, Semaphore};
use tokio_tungstenite::MaybeTlsStream;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use crate::base::AppError;

pub enum WsStream {
    Server(WebSocketStream<TcpStream>),
    Client(WebSocketStream<MaybeTlsStream<TcpStream>>),
}

pub enum WsReader {
    Server(SplitStream<WebSocketStream<TcpStream>>),
    Client(SplitStream<WebSocketStream<MaybeTlsStream<TcpStream>>>),
}

pub enum WsWriter {
    Server(SplitSink<WebSocketStream<TcpStream>, Message>),
    Client(SplitSink<WebSocketStream<MaybeTlsStream<TcpStream>>, Message>),
}

pub struct CommandResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
}

pub struct MessageManager {
    reader: Arc<Mutex<Option<WsReader>>>,
    writer: Arc<Mutex<Option<WsWriter>>>,
}

static INSTANCE: LazyLock<MessageManager> = LazyLock::new(MessageManager::new);

impl MessageManager {
    fn new() -> Self {
        Self {
            reader: Arc::new(Mutex::new(None)),
            writer: Arc::new(Mutex::new(None)),
        }
    }

    pub fn instance() -> &'static Self {
        &INSTANCE
    }

    pub async fn init(&self, ws_stream: WsStream) {
        match ws_stream {
            WsStream::Client(stream) => {
                let (tx, rx) = stream.split();
                self.reader.lock().await.replace(WsReader::Client(rx));
                self.writer.lock().await.replace(WsWriter::Client(tx));
            }
            WsStream::Server(stream) => {
                let (tx, rx) = stream.split();
                self.reader.lock().await.replace(WsReader::Server(rx));
                self.writer.lock().await.replace(WsWriter::Server(tx));
            }
        }
    }

    pub async fn dispose(&self) {
        *self.reader.lock().await = None;
        *self.writer.lock().await = None;
    }

    pub async fn run_shell(&self, script: &str) -> Result<CommandResult, AppError> {
        let output = Command::new("/bin/sh")
            .arg("-c")
            .arg(script)
            .output()
            .await?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let exit_code = output.status.code().unwrap_or(-1);

        Ok(CommandResult {
            stdout,
            stderr,
            exit_code,
        })
    }

    pub async fn process_messages(&self) -> Result<(), AppError> {
        if self.reader.lock().await.is_none() {
            return Err("WebSocket reader is not initialized".into());
        }
        loop {
            let next_msg = {
                let mut reader = self.reader.lock().await;
                match reader.as_mut() {
                    None => break,
                    Some(WsReader::Client(reader)) => reader.next().await,
                    Some(WsReader::Server(reader)) => reader.next().await,
                }
            };
            match next_msg {
                None => break,
                Some(Ok(Message::Close(_))) => break,
                Some(Err(e)) => return Err(e.into()),
                Some(Ok(msg)) => {
                    match msg {
                        Message::Text(text) => {
                            println!("📥 收到文本消息: {}", text);
                            let data = text.to_string();
                            let _ = MessageManager::instance().run_shell(&data).await;
                        }
                        Message::Binary(data) => {
                            println!("📦 收到二进制数据 ({} 字节)", data.len());
                        }
                        Message::Ping(data) => {
                            println!("🏓 收到 Ping ({} 字节)", data.len());
                        }
                        Message::Pong(data) => {
                            println!("🏓 收到 Pong ({} 字节)", data.len());
                        }
                        Message::Close(_) => {
                            println!("🚪 收到关闭帧");
                        }
                     _ => {}
                    };
                }
            }
        }
        Ok(())
    }

    pub async fn send_event(&self, data: String) -> Result<(), AppError> {
        MessageManager::instance().send(Message::Text(data.into())).await
    }
    
    pub async fn send(&self, msg: Message) -> Result<(), AppError> {
        let mut writer_guard = self.writer.lock().await;

        let Some(writer) = &mut *writer_guard else {
            return Err("WebSocket writer is not initialized".into());
        };

        println!("📥 发送事件: {}", msg.to_string());

        match writer {
            WsWriter::Client(w) => w.send(msg).await?,
            WsWriter::Server(w) => w.send(msg).await?,
        }
        
        Ok(())
    }
}