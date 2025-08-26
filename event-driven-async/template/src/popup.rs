use crossterm::event::KeyEvent;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
};

pub(crate) trait Popup {
    fn draw(&mut self, area: Rect, buf: &mut Buffer);

    fn handle_key_event(&mut self, key_event: KeyEvent);

    fn is_done(&self) -> bool;
}

pub(crate) fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}
