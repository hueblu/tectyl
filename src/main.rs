#![feature(panic_update_hook)]
#![allow(dead_code)]
#![allow(unused)]

use std::fs::File;
use tracing::{warn, Level};

use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use tectyl::{
    editor::Editor,
    term::{tui::Tui, Terminal},
};

#[tokio::main]
async fn main() -> Result<()> {
    init_logging()?;

    std::panic::update_hook(move |prev, info| {
        tracing::error!(?info, "THREAD PANICKED");
        let _ = Terminal::stop();

        println!("{info:?}");
        prev(info);
    });

    let mut editor = Editor::new()?;
    let mut terminal = Terminal::new()?;

    loop {
        // tui.draw()
        // tui.flush()

        warn!("penis");

        match terminal.recv_event().await {
            None => break,
            Some(Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers,
                ..
            })) if modifiers.contains(KeyModifiers::CONTROL) => panic!("balls"),
            Some(Event::Key(keyevent)) => editor.handle_event(keyevent)?,

            _ => {}
        }
    }

    Ok(())
}

fn init_logging() -> Result<()> {
    let open_file = File::options()
        .append(false)
        .write(true)
        .create(true)
        .open("log")?;

    tracing_subscriber::fmt()
        .with_max_level(Level::INFO)
        .with_writer(open_file)
        .init();

    Ok(())
}
