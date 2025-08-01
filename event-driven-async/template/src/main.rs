use crate::app::App;

pub(crate) mod app;
pub(crate) mod event;
pub(crate) mod ui;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let terminal = ratatui::init();
    let result = App::new().run(terminal).await;
    ratatui::restore();
    result
}
