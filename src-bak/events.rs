use tokio::sync::mpsc;

pub enum AppEvent {
    Input(crossterm::event::Event),
    RequestResult(String), // 假设请求返回一个 String
}
#[derive(Debug)]
pub struct MessageBus {
    tx: mpsc::Sender<AppEvent>,
    rx: mpsc::Receiver<AppEvent>,
}

impl MessageBus {
    pub fn new() -> Self {
        let (tx, rx) = mpsc::channel(32);
        Self { tx, rx }
    }

    pub fn get_sender(&self) -> mpsc::Sender<AppEvent> {
        self.tx.clone()
    }

    // 异步接收消息（必须用 `async`）
    pub async fn receive(&mut self) -> Option<AppEvent> {
        self.rx.recv().await
    }
}
