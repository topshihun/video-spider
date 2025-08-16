use ratatui::{
    Frame,
    layout::Rect,
    widgets::{List, ListState},
};

use crate::state::SeriesTabState;

pub struct SeriesPage {
    list_state: ListState,
}

impl SeriesPage {
    pub fn new() -> Self {
        Self {
            list_state: ListState::default(),
        }
    }

    pub fn draw(&self, frame: &mut Frame, area: Rect, state: &SeriesTabState) {
        if let Some(series) = state.get() {
            let items: Vec<String> = series.episodes.iter().map(|i| i.name.clone()).collect();
            let list = List::new(items);
            frame.render_widget(list, area);
        }
    }
}
