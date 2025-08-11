mod home_page;
mod search_page;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{layout::Rect, Frame};

use home_page::HomePage;
use search_page::SearchPage;
use crate::{state::{FocusState, State, UiState}, utils::style_block};

pub struct Page {
    home_page: HomePage,
    search_page: SearchPage,
}

impl Page {
    pub fn new() -> Self {
        Self {
            home_page: HomePage::new(),
            search_page: SearchPage::new(),
        }
    }

    pub fn draw(&self, frame: &mut Frame, area: Rect, state: &State) {
        let block = if let FocusState::Page = state.focus {
            style_block("page", true)
        } else {
            style_block("page", false)
        };

        match state.ui {
            UiState::Home => self.home_page.draw(frame, block.inner(area), state),
            UiState::Search => self.search_page.draw(frame, block.inner(area), state),
        }

        frame.render_widget(block, area);
    }

    pub fn handel_key_event(&mut self, key_event: KeyEvent, state: &mut State) {
        match key_event.code {
            KeyCode::Esc => state.focus.escape(),
            _ => {},
        }
    }
}
