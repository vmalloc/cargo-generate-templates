use crate::event::{AppEvent, Event, EventHandler};
use ratatui::{
    DefaultTerminal,
    crossterm::event::{KeyCode, KeyEvent, KeyModifiers},
};

#[derive(Debug, Clone, PartialEq, Default)]
pub(crate) enum AppState {
    #[default]
    Main,
    Help,
}

/// Application.
#[derive(Debug)]
pub(crate) struct App {
    /// Is the application running?
    pub(crate) running: bool,
    /// Current application state
    pub(crate) state: AppState,
    /// Event handler.
    pub(crate) events: EventHandler,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            state: AppState::default(),
            events: EventHandler::default(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub(crate) fn new() -> Self {
        Self::default()
    }

    /// Run the application's main loop.
    pub(crate) async fn run(mut self, mut terminal: DefaultTerminal) -> anyhow::Result<()> {
        while self.running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => {
                    if let crossterm::event::Event::Key(key_event) = event {
                        self.handle_key_events(key_event)?
                    }
                }
                Event::App(app_event) => match app_event {
                    AppEvent::Quit => self.quit(),
                },
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    pub(crate) fn handle_key_events(&mut self, key_event: KeyEvent) -> anyhow::Result<()> {
        match self.state {
            AppState::Main => match key_event.code {
                KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
                KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                    self.events.send(AppEvent::Quit)
                }
                KeyCode::Char('h' | '?') => {
                    self.state = AppState::Help;
                }
                _ => {}
            },
            AppState::Help => match key_event.code {
                KeyCode::Esc => {
                    self.state = AppState::Main;
                }
                KeyCode::Char('q') => self.events.send(AppEvent::Quit),
                KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                    self.events.send(AppEvent::Quit)
                }
                _ => {}
            },
        }
        Ok(())
    }

    /// Handles the tick event of the terminal.
    ///
    /// The tick event is where you can update the state of your application with any logic that
    /// needs to be updated at a fixed frame rate. E.g. polling a server, updating an animation.
    pub(crate) fn tick(&self) {}

    /// Set running to false to quit the application.
    pub(crate) fn quit(&mut self) {
        self.running = false;
    }
}
