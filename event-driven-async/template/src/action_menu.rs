use bon::Builder;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    text::{Line, Span},
    widgets::{List, ListState, StatefulWidget, Widget},
};

use crate::popup::{Popup, popup_area};

type ActionCallback = Box<dyn Fn() + 'static + Send + Sync>;

#[derive(Builder)]
pub(crate) struct ActionItem {
    shortcut: Option<char>,
    #[builder(with = |x: impl Fn() + Send + Sync + 'static| Box::new(x))]
    action: ActionCallback,

    #[builder(into)]
    title: String,
}

pub(crate) struct ActionMenu {
    title: Option<&'static str>,
    list: List<'static>,
    actions: Vec<ActionCallback>,
    shortcuts: Vec<Option<char>>,
    state: ListState,
    done: bool,
}

impl ActionMenu {
    pub(crate) fn new(title: Option<&'static str>, items: Vec<ActionItem>) -> Self {
        let mut actions = Vec::with_capacity(items.len());
        let mut shortcuts = Vec::with_capacity(items.len());
        let list = List::from_iter(items.into_iter().map(|item| {
            actions.push(item.action);
            shortcuts.push(item.shortcut);
            let mut spans = Vec::with_capacity(3);
            if let Some(sc) = item.shortcut {
                spans.push(Span::styled(
                    format!("({sc}) "),
                    crate::theme::shortcut_hint_style(),
                ));
            }
            spans.push(Span::styled(item.title, crate::theme::popup_item_style()));

            Line::from(spans)
        }))
        .highlight_style(crate::theme::highlighted_style());

        Self {
            title,
            actions,
            shortcuts,
            list,
            state: ListState::default(),
            done: false,
        }
    }

    fn dispatch(&mut self) {
        if let Some(selected) = self.state.selected() {
            self.actions[selected]();
            self.done = true
        }
    }

    fn dispatch_shortcut(&mut self, c: char) {
        for (idx, sc) in self.shortcuts.iter().enumerate() {
            if *sc == Some(c) {
                self.actions[idx]();
                self.done = true;
                return;
            }
        }
    }
}

impl Popup for ActionMenu {
    fn handle_key_event(&mut self, evt: KeyEvent) {
        match evt.code {
            KeyCode::Esc => self.done = true,
            KeyCode::Up => self.state.select_previous(),
            KeyCode::Down => self.state.select_next(),
            KeyCode::Enter => self.dispatch(),
            KeyCode::Char(c) if self.shortcuts.contains(&Some(c)) => self.dispatch_shortcut(c),
            _ => (),
        }
    }

    fn is_done(&self) -> bool {
        self.done
    }

    fn draw(&mut self, area: Rect, buf: &mut Buffer) {
        let area = popup_area(area, 50, 50);

        let block = ratatui::widgets::Block::bordered().title(self.title.unwrap_or("Popup"));

        ratatui::widgets::Clear.render(area, buf);
        (&block).render(area, buf);
        StatefulWidget::render(&self.list, block.inner(area), buf, &mut self.state);
    }
}
