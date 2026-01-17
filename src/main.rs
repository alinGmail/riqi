mod config;

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use serde::Deserialize;
use std::{io::{self, stdout}, sync::mpsc, time::Duration};

// 定义接口返回的数据结构
#[derive(Deserialize, Debug, Clone)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

// 消息类型，用于异步任务与 UI 线程通信
enum AppMessage {
    Loading,
    Loaded(Todo),
    Error(String),
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // 1. 初始化终端
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    // 2. 建立频道用于接收异步数据
    let (tx, rx) = mpsc::channel();

    // 3. 启动异步任务
    let tx_clone = tx.clone();
    tokio::spawn(async move {
        tx_clone.send(AppMessage::Loading).unwrap();
        // 模拟网络延迟
        tokio::time::sleep(Duration::from_secs(2)).await;
        
        match reqwest::get("https://jsonplaceholder.typicode.com/todos/1").await {
            Ok(resp) => {
                if let Ok(todo) = resp.json::<Todo>().await {
                    tx_clone.send(AppMessage::Loaded(todo)).unwrap();
                }
            }
            Err(e) => {
                tx_clone.send(AppMessage::Error(e.to_string())).unwrap();
            }
        }
    });

    // 4. UI 状态
    let mut status_text = String::from("初始化...");
    let mut current_todo: Option<Todo> = None;

    // 5. 主循环
    loop {
        terminal.draw(|f| {
            let area = f.size();
            
            // 构建显示内容
            let content = if let Some(todo) = &current_todo {
                format!(
                    "ID: {}\n标题: {}\n状态: {}",
                    todo.id,
                    todo.title,
                    if todo.completed { "已完成" } else { "未完成" }
                )
            } else {
                status_text.clone()
            };

            let block = Block::default()
                .title(" Ratatui 异步加载示例 ")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Cyan));

            let paragraph = Paragraph::new(content)
                .block(block)
                .alignment(Alignment::Center)
                .wrap(Wrap { trim: true });

            f.render_widget(paragraph, area);
        })?;

        // 检查是否有异步消息到达 (非阻塞)
        if let Ok(msg) = rx.try_recv() {
            match msg {
                AppMessage::Loading => status_text = "正在请求网络数据，请稍候...".to_string(),
                AppMessage::Loaded(todo) => current_todo = Some(todo),
                AppMessage::Error(e) => status_text = format!("错误: {}", e),
            }
        }

        // 事件处理：按 'q' 退出
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    // 6. 恢复终端
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}
