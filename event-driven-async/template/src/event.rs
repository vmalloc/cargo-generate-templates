use futures::{FutureExt, StreamExt};
use ratatui::crossterm::event::Event as CrosstermEvent;
use std::time::Duration;
use tokio::sync::mpsc;

/// Representation of all possible events.
#[derive(Clone, Debug)]
pub(crate) enum Event {
    /// An event that is emitted on a regular schedule.
    ///
    /// Use this event to run any code which has to run outside of being a direct response to a user
    /// event. e.g. polling exernal systems, updating animations, or rendering the UI based on a
    /// fixed frame rate.
    Tick,
    /// Crossterm events.
    ///
    /// These events are emitted by the terminal.
    Crossterm(CrosstermEvent),
    /// Application events.
    ///
    /// Use this event to emit custom events that are specific to your application.
    App(AppEvent),
}

/// Application events.
///
/// You can extend this enum with your own custom events.
#[derive(Clone, Debug)]
pub(crate) enum AppEvent {
    /// Quit the application.
    Quit,
}

/// Terminal event handler.
#[derive(Debug)]
pub(crate) struct EventHandler {
    /// Event sender channel.
    sender: mpsc::UnboundedSender<Event>,
    /// Event receiver channel.
    receiver: mpsc::UnboundedReceiver<Event>,
}

impl Default for EventHandler {
    fn default() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();
        let actor = EventTask::new(sender.clone());
        tokio::spawn(async { actor.run().await });
        Self { sender, receiver }
    }
}

impl EventHandler {
    /// Receives an event from the sender.
    ///
    /// This function blocks until an event is received.
    ///
    /// # Errors
    ///
    /// This function returns an error if the sender channel is disconnected. This can happen if an
    /// error occurs in the event thread. In practice, this should not happen unless there is a
    /// problem with the underlying terminal.
    pub(crate) async fn next(&mut self) -> anyhow::Result<Event> {
        self.receiver
            .recv()
            .await
            .ok_or_else(|| anyhow::anyhow!("Failed to receive event"))
    }

    /// Queue an app event to be sent to the event receiver.
    ///
    /// This is useful for sending events to the event handler which will be processed by the next
    /// iteration of the application's event loop.
    pub(crate) fn send(&mut self, app_event: AppEvent) {
        let _ = self.sender.send(Event::App(app_event));
    }

    pub(crate) fn redraw(&self) {
        let _ = self.sender.send(Event::Tick);
    }

    pub(crate) fn redrawer(&self) -> Redraw {
        Redraw {
            sender: self.sender.clone(),
        }
    }
}

#[derive(Clone)]
pub(crate) struct Redraw {
    sender: mpsc::UnboundedSender<Event>,
}

impl Redraw {
    pub(crate) fn redraw(&self) {
        let _ = self.sender.send(Event::Tick);
    }

    pub(crate) fn fps(&self, rate: f64) -> Fps {
        let sender = self.sender.clone();
        let (cancel_tx, mut cancel_rx) = tokio::sync::oneshot::channel::<()>();

        tokio::spawn(async move {
            let tick_rate = Duration::from_secs_f64(1.0 / rate);
            let mut interval = tokio::time::interval(tick_rate);
            loop {
                tokio::select! {
                    _ = &mut cancel_rx => {
                        break;
                    }
                    _ = interval.tick() => {
                        if sender.send(Event::Tick).is_err() {
                            break;
                        }
                    }
                }
            }
        });

        Fps { _cancel: cancel_tx }
    }
}

pub(crate) struct Fps {
    _cancel: tokio::sync::oneshot::Sender<()>,
}

/// A thread that handles reading crossterm events.
struct EventTask {
    /// Event sender channel.
    sender: mpsc::UnboundedSender<Event>,
}

impl EventTask {
    /// Constructs a new instance of [`EventTask`].
    fn new(sender: mpsc::UnboundedSender<Event>) -> Self {
        Self { sender }
    }

    /// Runs the event thread.
    ///
    /// This function polls for crossterm events and forwards them to the event receiver.
    async fn run(self) -> anyhow::Result<()> {
        let mut reader = crossterm::event::EventStream::new();
        loop {
            let crossterm_event = reader.next().fuse();
            tokio::select! {
              _ = self.sender.closed() => {
                break;
              }
              Some(Ok(evt)) = crossterm_event => {
                self.send(Event::Crossterm(evt));
              }
            };
        }
        Ok(())
    }

    /// Sends an event to the receiver.
    fn send(&self, event: Event) {
        let _ = self.sender.send(event);
    }
}
