use crossterm::event::{KeyCode, KeyEvent};
use ratatui::widgets::{Block, Paragraph};
use ratatui::{layout::Rect, Frame};
use ratatui::layout::Constraint;
use ratatui::{text::Text, layout::Layout};

use crate::{state::{FocusState, State, UiState}, utils::style_block};

pub struct Page {
    search_input: String,
}

impl Page {
    pub fn new() -> Self {
        Self {
            search_input: String::new(),
        }
    }

    pub fn draw(&self, frame: &mut Frame, area: Rect, state: &State) {
        let block = if let FocusState::Page = state.focus {
            style_block("page", true)
        } else {
            style_block("page", false)
        };

        match state.ui {
            UiState::Home => self.draw_home_page(frame, block.inner(area), state),
            UiState::Search => self.draw_search_page(frame, block.inner(area), state),
        }

        frame.render_widget(block, area);
    }

    fn draw_home_page(&self, frame: &mut Frame, area: Rect, _state: &State) {
        let text = Text::from("Home page is nothing");
        frame.render_widget(text, area);
    }

    fn draw_search_page(&self, frame: &mut Frame, area: Rect, _state: &State) {
        let chunks = Layout::vertical([
            Constraint::Length(3),
        ])
            .margin(1)
            .split(area);

        // draw search input
        let search_input = Paragraph::new(self.search_input.as_str())
            .block(Block::bordered());
        frame.render_widget(search_input, chunks[0]);
    }

    pub fn handel_key_event(&mut self, key_event: KeyEvent, state: &mut State) {
        match key_event.code {
            KeyCode::Esc => state.focus.escape(),
            _ => {},
        }
    }
}
