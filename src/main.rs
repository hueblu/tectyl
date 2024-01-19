use crossterm::ExecutableCommand;
use crossterm::QueueableCommand;
use std::io::Write;

use anyhow::Result;
use crossterm::{
    cursor::MoveTo,
    event::{read, Event, KeyCode, KeyEvent, KeyModifiers},
    terminal::{Clear, ClearType, LeaveAlternateScreen},
};

use tectyl::{editor::Editor, tui::Terminal};

#[tokio::main]
async fn main() -> Result<()> {
    let mut terminal = Terminal::new()?;

    let mut editor = Editor::new();

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

        terminal.out.queue(MoveTo(0, 0))?;
        terminal.out.queue(Clear(ClearType::All))?;

        editor.draw(&mut terminal)?;
        terminal.out.flush()?;
    }

    terminal.out.execute(LeaveAlternateScreen)?;
    crossterm::terminal::disable_raw_mode()?;

    Ok(())
}
