#![feature(panic_update_hook)]

use crossterm::QueueableCommand;
use std::fs::File;
use std::io::Write;
use tracing::Level;

use anyhow::Result;
use crossterm::{
    cursor::MoveTo,
    event::{Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{Clear, ClearType},
};

use tectyl::{editor::Editor, term::Terminal};

#[tokio::main]
async fn main() -> Result<()> {
    init_logging()?;

    let mut terminal = Terminal::new()?;

    std::panic::update_hook(move |prev, info| {
        tracing::error!(?info, "THREAD PANICKED");
        let _ = Terminal::stop();
        prev(info);
    });

    tracing::error!("bussknuckle");

    let mut editor = Editor::new().await?;

    editor.draw(&mut terminal)?;
    terminal.flush()?;

    loop {
        match terminal.recv_event().await {
            None => break,
            Some(Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers,
                ..
            })) if modifiers.contains(KeyModifiers::CONTROL) => break,
            Some(Event::Key(keyevent)) => editor.handle_event(keyevent),

            _ => {}
        }

        terminal.queue(MoveTo(0, 0))?;
        terminal.queue(Clear(ClearType::All))?;

        editor.draw(&mut terminal)?;
        terminal.flush()?;
    }

    terminal.exit()?;

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
