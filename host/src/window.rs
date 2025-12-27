use anyhow::Result;
use wry::raw_window_handle::HasWindowHandle;
use wry::{WebView, WebViewBuilder};

pub struct AppWindow {
    pub webview: WebView,
}

impl AppWindow {
    pub fn new<W: HasWindowHandle>(
        window: &W,
        url: &str,
        _title: &str,
        _width: u32,
        _height: u32,
    ) -> Result<Self> {
        let webview = WebViewBuilder::new().with_url(url).build(window)?;

        Ok(Self { webview })
    }

    pub fn send_message(&self, message: &str) -> Result<()> {
        self.webview.evaluate_script(message)?;
        Ok(())
    }
}
