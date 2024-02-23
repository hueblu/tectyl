#![feature(panic_update_hook)]
#![allow(dead_code)]
#![allow(unused)]

use ratatui::Terminal;
use std::fs::File;
use tracing::{info, trace, Level};

use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};

use tectyl::editor::Editor;

#[tokio::main]
async fn main() -> Result<()> {
    init_logging(4)?;

    std::panic::update_hook(move |prev, info| {
        tracing::error!(?info, "THREAD PANICKED");

        println!("{info:?}");
        prev(info);
    });

    let mut editor = Editor::new()?;
    let mut tui = Tui::new();

    loop {
        tui.draw(&editor);
        // tui.flush()

        match tui.recv_event().await {
            None => break,
            Some(Event::Key(KeyEvent {
                code: KeyCode::Char('c'),
                modifiers,
                ..
            })) if modifiers.contains(KeyModifiers::CONTROL) => {
                info!("exited on Ctrl-c");
                break;
            }
            Some(Event::Key(keyevent)) => {
                trace!(event = ?keyevent, "recieved key event");
                editor.handle_event(keyevent)?
            }

            _ => {}
        }
    }

    Ok(())
}

fn init_logging(verbosity: u8) -> Result<()> {
    let open_file = File::options()
        .append(false)
        .write(true)
        .create(true)
        .open("log")?;
    open_file.set_len(0);

    tracing_subscriber::fmt()
        .with_max_level(match verbosity {
            1 => Level::ERROR,
            2 => Level::WARN,
            3 => Level::INFO,
            4 => Level::DEBUG,
            _ => Level::TRACE,
        })
        .with_writer(open_file)
        .init();

    Ok(())
}
