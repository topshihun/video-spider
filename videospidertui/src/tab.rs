use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
};

use crate::{
    state::{FocusState, State, TabState},
    utils::{style_block, style_text},
};

pub struct Tab {}

impl Tab {
    pub fn new() -> Tab {
        Self {}
    }

    pub fn draw(&self, frame: &mut Frame, area: Rect, state: (&TabState, &FocusState)) {
        let (tab_state, focus_state) = state;

        let block = if let FocusState::Tab = focus_state {
            style_block("tab", true)
        } else {
            style_block("tab", false)
        };
        frame.render_widget(block, area);

        let chunks_main = Layout::vertical([Constraint::Length(1), Constraint::Length(1)])
            .margin(1)
            .split(area);

        let home = if let TabState::Home = tab_state {
            style_text("home", true)
        } else {
            style_text("home", false)
        };
        frame.render_widget(home, chunks_main[0]);

        let search = if let TabState::Search = tab_state {
            style_text("search", true)
        } else {
            style_text("search", false)
        };
        frame.render_widget(search, chunks_main[1]);
    }

    pub fn handel_key_event(&mut self, key_event: KeyEvent, state: &mut State) {
        match key_event.code {
            KeyCode::Enter => state.focus_state.enter(),
            KeyCode::Char('j') => {
                state.tab_state.write().unwrap().next();
                state.update_page_state();
            }
            KeyCode::Char('k') => {
                state.tab_state.write().unwrap().prev();
                state.update_page_state();
            }
            KeyCode::Char('h') => {
                state.focus_state.prev();
                state.update_page_state();
            }
            KeyCode::Char('l') => {
                state.focus_state.next();
                state.update_page_state();
            }
            _ => {}
        }
    }
}
