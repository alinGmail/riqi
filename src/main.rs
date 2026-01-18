mod config;
mod ui;
mod state;
mod data;
mod theme;

use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{prelude::*, widgets::*};
use serde::Deserialize;
use std::{io::{self, stdout}, sync::mpsc, thread, time::Duration};

#[derive(Deserialize, Debug, Clone)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

// 统一的事件枚举：合并了 UI 事件和业务数据事件
enum AppEvent {
    Quit,
    TerminalEvent(Event),
    DataLoaded(Todo),
    LoadError(String),
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // --- 1. 初始化终端 ---
    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;

    // --- 2. 创建核心事件通道 ---
    let (tx, rx) = mpsc::channel();

    // 事件源 A: 终端输入监听线程 (将 crossterm 事件转发到 mpsc)
    let tx_input = tx.clone();
    thread::spawn(move || {
        loop {
            if event::poll(Duration::from_millis(500)).unwrap() {
                if let Ok(ev) = event::read() {
                    if let Event::Key(key) = ev {
                        if key.code == KeyCode::Char('q') {
                            let _ = tx_input.send(AppEvent::Quit);
                            break;
                        }
                    }
                    let _ = tx_input.send(AppEvent::TerminalEvent(ev));
                }
            }
        }
    });

    // 事件源 B: 异步网络请求
    let tx_net = tx.clone();
    tokio::spawn(async move {
        // 模拟网络延迟
        tokio::time::sleep(Duration::from_secs(2)).await;
        let url = "https://jsonplaceholder.typicode.com/todos/1";
        
        match reqwest::get(url).await {
            Ok(resp) => {
                if let Ok(todo) = resp.json::<Todo>().await {
                    let _ = tx_net.send(AppEvent::DataLoaded(todo));
                }
            }
            Err(e) => {
                let _ = tx_net.send(AppEvent::LoadError(e.to_string()));
            }
        }
    });

    // --- 3. 状态与主循环 ---
    let mut todo_data: Option<Todo> = None;
    let mut error_msg: Option<String> = None;

    // 初始手动触发一次渲染（显示“加载中”）
    draw_ui(&mut terminal, &todo_data, &error_msg)?;

    loop {
        // 【关键】阻塞式接收：没有事件时，程序会停留在此处，不消耗 CPU
        match rx.recv().unwrap() {
            AppEvent::Quit => break,
            
            AppEvent::DataLoaded(todo) => {
                todo_data = Some(todo);
                // 收到数据，触发重绘
                draw_ui(&mut terminal, &todo_data, &error_msg)?;
            }
            
            AppEvent::LoadError(e) => {
                error_msg = Some(e);
                // 发生错误，触发重绘
                draw_ui(&mut terminal, &todo_data, &error_msg)?;
            }

            AppEvent::TerminalEvent(Event::Resize(_, _)) => {
                // 窗口大小改变，触发重绘
                draw_ui(&mut terminal, &todo_data, &error_msg)?;
            }
            
            _ => {} // 其他按键暂不触发重绘
        }
    }

    // --- 4. 恢复终端 ---
    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;
    Ok(())
}

// 将渲染逻辑抽离
fn draw_ui(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    todo: &Option<Todo>,
    err: &Option<String>
) -> io::Result<()> {
    terminal.draw(|f| {
        let area = f.size();
        
        let content = if let Some(e) = err {
            format!("错误: {}", e)
        } else if let Some(t) = todo {
            format!("ID: {}\n标题: {}\n状态: {}", t.id, t.title, if t.completed { "完成" } else { "未完成" })
        } else {
            "正在异步加载数据 (JSONPlaceholder)...".to_string()
        };

        let block = Block::default()
            .title(" 事件驱动渲染示例 (按 'q' 退出) ")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(if todo.is_some() { Color::Green } else { Color::Yellow }));

        let paragraph = Paragraph::new(content)
            .block(block)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        f.render_widget(paragraph, area);
    })?;
    Ok(())
}
