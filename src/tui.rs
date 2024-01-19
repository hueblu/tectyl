use anyhow::Result;
use crossterm::{
    event::{Event, EventStream},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use futures::{future::FutureExt, StreamExt};
use tokio::{sync::mpsc, task::JoinHandle};

use std::io::{stdout, Stdout};

pub struct Terminal {
    size: (u16, u16),

    pub out: Stdout,
    events: mpsc::Receiver<Event>,

    thread: JoinHandle<()>,
}

impl Terminal {
    pub fn new() -> Result<Self> {
        let mut out = stdout();

        terminal::enable_raw_mode()?;
        out.execute(EnterAlternateScreen)?;

        let size = terminal::size()?;

        let (tx, rx) = mpsc::channel(20);
        let thread = tokio::spawn(async move {
            let mut event_stream = EventStream::new();

            loop {
                let event_future = event_stream.next();

                tokio::select! {
                    maybe_event = event_future => {
                        match maybe_event {
                            Some(Ok(event)) => { let _ = tx.send(event).await; }
                            None => { break; }
                            _ => {}
                        }
                    }
                }
            }
        });

        Ok(Self {
            size,
            out,
            events: rx,
            thread,
        })
    }

    pub async fn recv_event(&mut self) -> Option<Event> {
        self.events.recv().await
    }

    pub fn size(&self) -> (u16, u16) {
        self.size
    }

    pub fn exit(&mut self) -> Result<()> {
        self.out.execute(LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        let _ = self.exit();
    }
}
