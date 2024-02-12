use anyhow::Result;
use crossterm::{
    cursor::MoveTo,
    event::{Event, EventStream},
    style::Print,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand, QueueableCommand,
};
use futures::StreamExt;
use tokio::{
    sync::{mpsc, Mutex},
    task::JoinHandle,
};
use tokio_util::sync::CancellationToken;

use std::{
    io::{stdout, Stdout},
    ops::{Deref, DerefMut},
    sync::Arc,
};

use super::util::{ScreenBuffer, ScreenBufferDiff};

pub struct Terminal {
    size: Arc<Mutex<(usize, usize)>>,

    out: Stdout,
    events: mpsc::Receiver<Event>,

    thread: JoinHandle<()>,
    cancel: CancellationToken,
}

impl Terminal {
    pub fn new() -> Result<Self> {
        let mut out = stdout();

        terminal::enable_raw_mode()?;
        out.execute(EnterAlternateScreen)?;

        let size = terminal::size()?;
        let size = Arc::new(Mutex::new((size.0 as usize, size.1 as usize)));
        let size_clone = size.clone();

        let (tx, rx) = mpsc::channel(20);
        let token = CancellationToken::new();
        let token_clone = token.clone();
        let thread = tokio::spawn(async move {
            let mut event_stream = EventStream::new();

            loop {
                let event_future = event_stream.next();
                let cancel = token.cancelled();

                tokio::select! {
                    maybe_event = event_future => {
                        match maybe_event {
                            Some(Ok(event)) => {
                                if let Event::Resize(x, y) = event {
                                    *size_clone.lock().await = (x as usize, y as usize);
                                }
                                let _ = tx.send(event).await;

                            }
                            None => { break; }
                            _ => {}
                        }
                    }
                    _ = cancel => {
                        break;
                    }
                }
            }
        });

        Ok(Self {
            size,
            out,
            events: rx,

            thread,
            cancel: token_clone,
        })
    }

    pub async fn recv_event(&mut self) -> Option<Event> {
        self.events.recv().await
    }

    pub async fn size(&self) -> (usize, usize) {
        *self.size.lock().await
    }

    pub fn draw_buffer(&mut self, buf: ScreenBuffer) -> Result<()> {
        let lines = String::from_iter(buf.cells.clone());
        self.out.queue(MoveTo(0, 0))?;

        for line in lines.chars().take(buf.size().0) {
            self.out.queue(Print(line))?;
        }

        Ok(())
    }

    pub fn draw_buffer_diff(&self, buf: ScreenBufferDiff) -> Result<()> {
        Ok(())
    }

    pub fn exit(&mut self) -> Result<()> {
        self.cancel.cancel();

        self.thread.abort();

        self.out.execute(LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    pub fn stop() -> Result<()> {
        std::io::stdout().execute(LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;

        Ok(())
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        let _ = self.exit();
    }
}

impl Deref for Terminal {
    type Target = Stdout;

    fn deref(&self) -> &Self::Target {
        &self.out
    }
}

impl DerefMut for Terminal {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.out
    }
}
