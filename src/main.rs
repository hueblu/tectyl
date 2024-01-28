#![feature(panic_update_hook)]

use std::fs::File;
use tracing::Level;

use anyhow::Result;
use crossterm::{
    cursor::MoveTo,
    event::{Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{Clear, ClearType},
};

use tectyl::{
    editor::Editor,
    term::{tui::Tui, Terminal},
};

#[tokio::main]
async fn main() -> Result<()> {
    init_logging()?;

    let mut tui = Tui::new()?;

    std::panic::update_hook(move |prev, info| {
        tracing::error!(?info, "THREAD PANICKED");
        let _ = Terminal::stop();
        prev(info);
    });

    tracing::error!("bussknuckle");

    let mut editor = Editor::new().await?;

    loop {
        // tui.draw()
        // tui.flush()

        match tui.recv_event().await {
            None => break,
            Some(Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers,
                ..
            })) if modifiers.contains(KeyModifiers::CONTROL) => break,
            Some(Event::Key(keyevent)) => editor.handle_event(keyevent),

            _ => {}
        }
    }

    Ok(())
}

fn init_logging() -> Result<()> {
    let open_file = File::options().write(true).create(true).open("log")?;

    tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .with_writer(open_file)
        .init();

    Ok(())
}
