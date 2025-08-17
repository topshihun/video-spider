mod home_page;
mod search_page;
mod series_page;

use std::sync::mpsc::Sender;

use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect};

use crate::{
    message::Message,
    page::series_page::SeriesPage,
    state::{FocusState, PageState, State, TabState},
    utils::style_block,
};
use home_page::HomePage;
use search_page::SearchPage;

pub struct Page {
    home_page: HomePage,
    search_page: SearchPage,
    series_page: SeriesPage,
}

impl Page {
    pub fn new(sender: Sender<Message>) -> Self {
        Self {
            home_page: HomePage::new(),
            search_page: SearchPage::new(sender),
            series_page: SeriesPage::new(),
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, area: Rect, state: (&PageState, &FocusState)) {
        let (page_state, focus_state) = state;

        let block = if let FocusState::Page = focus_state {
            style_block("page", true)
        } else {
            style_block("page", false)
        };
        let block_inner = block.inner(area);
        frame.render_widget(block, area);

        match page_state {
            PageState::Tab(tab_state) => match *tab_state.read().unwrap() {
                TabState::Home => self.home_page.draw(frame, block_inner, page_state),
                TabState::Search => self.search_page.draw(frame, block_inner, page_state),
            },
            PageState::Series(series_tab_state) => {
                self.series_page
                    .draw(frame, block_inner, &series_tab_state.read().unwrap());
            }
        }

        if let PageState::Tab(tab_state) = page_state {
            match *tab_state.read().unwrap() {
                TabState::Home => self.home_page.draw(frame, block_inner, page_state),
                TabState::Search => self.search_page.draw(frame, block_inner, page_state),
            }
        }
    }

    pub fn handel_key_event(&mut self, key_event: KeyEvent, state: &mut State) {
        // TODO: remove page_state clone
        match state.page_state.clone() {
            PageState::Tab(tab_state) => match *tab_state.read().unwrap() {
                TabState::Home => self.home_page.handle_key_event(key_event, state),
                TabState::Search => self.search_page.handle_key_event(key_event, state),
            },
            PageState::Series(_) => {
                self.series_page.handle_key_event(key_event, state);
            }
        }
    }
}
