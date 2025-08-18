use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::Rect, text::Text};

use crate::state::{PageState, State};

pub struct HomePage {}

impl HomePage {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self, frame: &mut Frame, area: Rect, _state: &PageState) {
        let text = Text::from("Home page is nothing");
        frame.render_widget(text, area);
    }

    pub fn handle_key_event(&mut self, key_evnet: KeyEvent, state: &mut State) {
        match key_evnet.code {
            KeyCode::Esc | KeyCode::Char('q') => {
                state.focus_state.escape(&state.page_state);
            }
            _ => {}
        }
    }
}
