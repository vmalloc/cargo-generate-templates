use ratatui::style::{Style, Stylize as _};

pub(crate) fn shortcut_hint_style() -> Style {
    Style::new().magenta()
}

pub(crate) fn popup_item_style() -> Style {
    Default::default()
}

pub(crate) fn highlighted_style() -> Style {
    Style::new().black().on_white()
}
