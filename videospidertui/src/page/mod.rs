mod home_page;
mod search_page;

use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect};

use crate::{
    state::{FocusState, PageState, TabState},
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
        let block_inner = block.inner(area);
        frame.render_widget(block, area);

        if let PageState::Tab(tab_state) = page_state { match tab_state {
            TabState::Home => self.home_page.draw(frame, block_inner, page_state),
            TabState::Search => self.search_page.draw(frame, block_inner, page_state),
        } }
    }

    pub fn handel_key_event(&mut self, key_event: KeyEvent, state: (&PageState, &mut FocusState)) {
        let (page_state, focus_state) = state;

        if let PageState::Tab(tab_state) = page_state { match tab_state {
            TabState::Home => self.home_page.handle_key_event(key_event, focus_state),
            TabState::Search => self.search_page.handle_key_event(key_event, focus_state),
        } }
    }
}
