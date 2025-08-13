mod home_page;
mod search_page;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::Rect};

use crate::{
    state::{FocusState, PageState, State, TabState},
    utils::style_block,
};
use home_page::HomePage;
use search_page::SearchPage;

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

    pub fn draw(&self, frame: &mut Frame, area: Rect, state: (&PageState, &FocusState)) {
        let (page_state, focus_state) = state;

        let block = if let FocusState::Page = focus_state {
            style_block("page", true)
        } else {
            style_block("page", false)
        };

        match page_state {
            PageState::Tab(tab_state) => match tab_state {
                TabState::Home => self.home_page.draw(frame, block.inner(area), page_state),
                TabState::Search => self.search_page.draw(frame, block.inner(area), page_state),
            },
            _ => {}
        }

        frame.render_widget(block, area);
    }

    pub fn handel_key_event(&mut self, key_event: KeyEvent, state: &mut FocusState) {
        match key_event.code {
            KeyCode::Esc => state.escape(),
            _ => {}
        }
    }
}
