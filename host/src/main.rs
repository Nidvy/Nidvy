mod handler;
mod ipc;
mod window;

fn main() -> anyhow::Result<()> {
    let event_loop = EventLoop::new()?;
    let mut state = AppState::new();

    // stdin
    let (tx, rx) = std::sync::mpsc::channel::<IPCMessage>();

    std::thread::spawn(move || {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            if let Ok(line) = line {
                if let Ok(msg) = serde_json::from_str::<IPCMessage>(&line) {
                    tx.send(msg).ok();
                }
            }
        }
    });

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => {
                eprintln!("[host] ready");
            }

            Event::MainEventsCleared => {
                while let Ok(msg) = rx.try_recv() {
                    let resp = handle_message(msg, &mut state, &event_loop);
                    let json = serde_json::to_string(&resp).unwrap();
                    println!("{json}");
                    io::stdout().flush().ok();
                }
            }

            _ => {}
        }
    });
}
