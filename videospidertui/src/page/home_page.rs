use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::Rect, text::Text};

use crate::state::{FocusState, PageState, State};

pub struct HomePage {}

impl HomePage {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self, frame: &mut Frame, area: Rect, _state: &PageState) {
        let text = Text::from("Home page is nothing");
        frame.render_widget(text, area);
    }

    pub fn handle_key_event(&mut self, key_evnet: KeyEvent, state: &mut FocusState) {
        match key_evnet.code {
            KeyCode::Esc => state.escape(),
            _ => {}
        }
    }
}
