use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Stylize},
    widgets::{Block, BorderType, Padding, Paragraph, Widget},
};

use crate::app::{App, AppState};

impl Widget for &App {
    /// Renders the user interface widgets.
    ///
    // This is where you add new widgets.
    // See the following resources:
    // - https://docs.rs/ratatui/latest/ratatui/widgets/index.html
    // - https://github.com/ratatui/ratatui/tree/master/examples
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [main_area, status_area] = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(0),    // Main area
                Constraint::Length(1), // Status line
            ])
            .areas(area);

        match self.state {
            AppState::Main => {
                let block = Block::bordered()
                    .title("{{project-name}}")
                    .title_alignment(Alignment::Center)
                    .border_type(BorderType::Rounded);

                let text = "This is a tui template.\n\
                        Press `Esc`, `Ctrl-C` or `q` to stop running.\n\
                        Press `h` or `?` for help.";

                let paragraph = Paragraph::new(text)
                    .block(block)
                    .fg(Color::Cyan)
                    .bg(Color::Black)
                    .centered();

                paragraph.render(main_area, buf);

                let status_line = Paragraph::new("Main Screen | Press h/? for help")
                    .black()
                    .on_magenta();
                status_line.render(status_area, buf);
            }
            AppState::Help => {
                let block = Block::bordered()
                    .title("Help")
                    .padding(Padding::proportional(2))
                    .title_alignment(Alignment::Center)
                    .border_type(BorderType::Rounded);

                let help_text = "Key Bindings:\n\
                        • h, ?             - Show this help screen\n\
                        • Esc              - Return to main screen\n\
                        • q, Ctrl-C        - Quit application";

                let paragraph = Paragraph::new(help_text)
                    .block(block)
                    .fg(Color::Yellow)
                    .bg(Color::Black);

                paragraph.render(main_area, buf);

                let status_line = Paragraph::new("Help Screen | Press Esc to return")
                    .black()
                    .on_yellow();
                status_line.render(status_area, buf);
            }
        }
    }
}
