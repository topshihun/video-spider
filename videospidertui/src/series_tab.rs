use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{layout::Rect, Frame};

use crate::{state::{FocusState, State}, utils::style_block};

pub struct SeriesTab {}

impl SeriesTab {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn draw(&self, frame: &mut Frame, area: Rect, state: &State) {
        let block = if let FocusState::SeriesTab = state.focus {
            style_block("series", true)
        } else {
            style_block("series", false)
        };
        frame.render_widget(block, area);
    }

    pub fn handel_key_event(&mut self, key_event: KeyEvent, state: &mut State) {
        match key_event.code {
            KeyCode::Enter => state.focus.enter(),
            KeyCode::Char('h') => state.focus.prev(),
            KeyCode::Char('l') => state.focus.next(),
            _ => {},
        }
    }
}
