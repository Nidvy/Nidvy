use crate::{ipc::IPCMessage, window::AppWindow};
use anyhow::Result;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

pub struct AppState {
    pub window: Option<AppWindow>,
    pub event_loop: Option<EventLoop<()>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            window: None,
            event_loop: None,
        }
    }

    pub fn get_or_create_event_loop(&mut self) -> Result<&mut EventLoop<()>> {
        if self.event_loop.is_none() {
            self.event_loop = Some(EventLoop::new()?);
        }
        Ok(self.event_loop.as_mut().unwrap())
    }
}

fn create_window(
    state: &mut AppState,
    url: &str,
    title: &str,
    width: u32,
    height: u32,
) -> Result<()> {
    let event_loop = state.get_or_create_event_loop()?;

    let window: Window = WindowBuilder::new()
        .with_title(title)
        .with_inner_size(winit::dpi::LogicalSize::new(width, height))
        .build(event_loop)?;

    let app_window = AppWindow::new(&window, url, title, width, height)?;
    state.window = Some(app_window);

    Ok(())
}

fn error(id: Option<u64>, code: &str, message: &str) -> IPCMessage {
    IPCMessage {
        id,
        r#type: "error".to_string(),
        method: None,
        params: None,
        result: None,
        error: Some(crate::ipc::IPCError {
            code: -1,
            message: format!("{}: {}", code, message),
        }),
    }
}

pub fn handle_message(msg: IPCMessage, state: &mut AppState) -> IPCMessage {
    match msg.method.as_deref() {
        Some("window.create") => {
            let params = msg.params.unwrap_or_default();
            let url = params
                .get("url")
                .and_then(|v| v.as_str())
                .unwrap_or("https://www.baidu.com");
            let title = params
                .get("title")
                .and_then(|v| v.as_str())
                .unwrap_or("Nidvy");
            let width = params.get("width").and_then(|v| v.as_u64()).unwrap_or(800) as u32;
            let height = params.get("height").and_then(|v| v.as_u64()).unwrap_or(600) as u32;

            // 创建窗口和 WebView
            match create_window(state, url, title, width, height) {
                Ok(_) => IPCMessage {
                    id: msg.id,
                    r#type: "response".to_string(),
                    method: msg.method,
                    params: msg.params,
                    result: Some(serde_json::json!({"success": true})),
                    error: None,
                },
                Err(e) => error(msg.id, "WINDOW_CREATE_ERROR", &e.to_string()),
            }
        }
        _ => error(msg.id, "UNKNOWN_METHOD", "method not found"),
    }
}
