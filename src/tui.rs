use anyhow::Result;
use crossterm::{
    event::{Event, EventStream},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use futures::StreamExt;
use tokio::{sync::mpsc, task::JoinHandle};
use tokio_util::sync::CancellationToken;

use std::io::{stdout, Stdout};

pub struct Terminal {
    size: (u16, u16),

    pub out: Stdout,
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
                            Some(Ok(event)) => { let _ = tx.send(event).await; }
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

    pub fn size(&self) -> (u16, u16) {
        self.size
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
