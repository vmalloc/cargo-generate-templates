use crate::{
    action_menu::ActionItem,
    event::{AppEvent, Event, EventHandler},
    popup::Popup,
};
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
pub(crate) struct App {
    /// Is the application running?
    pub(crate) running: bool,
    /// Current application state
    pub(crate) state: AppState,
    /// Event handler.
    pub(crate) events: EventHandler,

    /// Optional popup being displayed
    pub(crate) popup: Option<Box<dyn Popup>>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            state: AppState::default(),
            events: EventHandler::default(),
            popup: None,
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
            terminal.draw(|frame| frame.render_widget(&mut self, frame.area()))?;
            match self.events.next().await? {
                Event::Tick => self.tick(),
                Event::Crossterm(event) => {
                    if let crossterm::event::Event::Key(key_event) = event {
                        self.handle_key_event(key_event)?
                    }
                }
                Event::App(app_event) => match app_event {
                    AppEvent::Quit => self.quit(),
                },
            }

            if let Some(popup) = self.popup.as_mut()
                && popup.is_done()
            {
                self.popup = None;
            }
        }
        Ok(())
    }

    pub(crate) fn show_popup<T: Popup + 'static>(&mut self, popup: T) {
        self.popup = Some(Box::new(popup));
    }

    /// Handles the key events and updates the state of [`App`].
    pub(crate) fn handle_key_event(&mut self, key_event: KeyEvent) -> anyhow::Result<()> {
        if let Some(popup) = self.popup.as_mut() {
            popup.handle_key_event(key_event);
        } else {
            match self.state {
                AppState::Main => match key_event.code {
                    KeyCode::Esc | KeyCode::Char('q') => self.events.send(AppEvent::Quit),
                    KeyCode::Char('c' | 'C') if key_event.modifiers == KeyModifiers::CONTROL => {
                        self.events.send(AppEvent::Quit)
                    }
                    KeyCode::Char('h' | '?') => {
                        self.state = AppState::Help;
                    }
                    KeyCode::Char('p') => {
                        self.show_popup(crate::action_menu::ActionMenu::new(
                            None,
                            vec![
                                ActionItem::builder()
                                    .title("Action 1")
                                    .action(|| {})
                                    .build(),
                                ActionItem::builder()
                                    .title("Action 2")
                                    .action(|| {})
                                    .build(),
                            ],
                        ));
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
