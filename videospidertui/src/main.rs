mod app;
mod page;
mod series_tab;
mod state;
mod tab;
mod utils;

fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let result = app::App::new().run(&mut terminal);
    ratatui::restore();
    result
}
