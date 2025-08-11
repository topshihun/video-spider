use ratatui::{layout::Rect, text::Text, Frame};

use crate::state::State;

pub struct HomePage {}

impl HomePage {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn draw(&self, frame: &mut Frame, area: Rect, _state: &State) {
        let text = Text::from("Home page is nothing");
        frame.render_widget(text, area);
    }
}
