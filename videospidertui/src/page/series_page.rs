use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::Rect,
    style::{Style, Stylize},
    widgets::{List, ListState},
};
use videospider::play;

use crate::state::{SeriesTabState, State};

pub struct SeriesPage {
    list_state: ListState,
}

impl SeriesPage {
    pub fn new() -> Self {
        Self {
            list_state: ListState::default(),
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, area: Rect, state: &SeriesTabState) {
        if let Some(series) = state.get() {
            let items: Vec<String> = series
                .episodes
                .iter()
                .map(|i| format!("{}: {}", i.name, i.addr))
                .collect();
            let list = List::new(items).highlight_style(Style::new().reversed());
            frame.render_stateful_widget(list, area, &mut self.list_state);
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent, state: &mut State) {
        match key_event.code {
            KeyCode::Char('j') => {
                self.list_state.select_next();
            }
            KeyCode::Char('k') => self.list_state.select_previous(),
            KeyCode::Enter => {
                if let Some(index) = self.list_state.selected()
                    && let Some(series) = state.series_tab_state.read().unwrap().get() {
                        let episode = series.episodes.get(index).unwrap();
                        play(episode).unwrap();
                    }
            }
            KeyCode::Esc => state.focus_state.escape(),
            _ => {}
        }
    }
}
