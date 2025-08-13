use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Paragraph},
};

use crate::state::{PageState, State};

pub struct SearchPage {
    search_input: String,
}

impl SearchPage {
    pub fn new() -> Self {
        Self {
            search_input: String::new(),
        }
    }

    pub fn draw(&self, frame: &mut Frame, area: Rect, _state: &PageState) {
        let chunks = Layout::vertical([Constraint::Length(3)])
            .margin(1)
            .split(area);

        // draw search input
        let search_input = Paragraph::new(self.search_input.as_str()).block(Block::bordered());
        frame.render_widget(search_input, chunks[0]);
    }
}
