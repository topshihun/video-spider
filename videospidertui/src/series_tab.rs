use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{layout::Rect, Frame};

use crate::{state::{FocusState, SeriesTabState, State}, utils::style_block};

pub struct SeriesTab {}

impl SeriesTab {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn draw(&self, frame: &mut Frame, area: Rect, state: (&SeriesTabState, &FocusState)) {
        let (series_tab_state, focus_state) = state;

        let block = if let FocusState::SeriesTab = focus_state {
            style_block("series", true)
        } else {
            style_block("series", false)
        };

        frame.render_widget(block, area);
    }

    pub fn handel_key_event(&mut self, key_event: KeyEvent, state: &mut FocusState) {
        match key_event.code {
            KeyCode::Enter => state.enter(),
            KeyCode::Char('h') => state.prev(),
            KeyCode::Char('l') => state.next(),
            _ => {},
        }
    }
}
