use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{layout::{Constraint, Layout, Rect}, Frame};

use crate::{state::{FocusState, State, UiState}, utils::{style_block, style_text}};

pub struct Tab {
}

impl Tab {
    pub fn new() -> Tab {
        Self {
        }
    }

    pub fn draw(&self, frame: &mut Frame, area: Rect, state: &State) {
        let block = if let FocusState::Tab = state.focus {
            style_block("tabs", true)
        } else {
            style_block("tabs", false)
        };
        frame.render_widget(block, area);

        let chunks_main = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
        ])
            .margin(1)
            .split(area);

        let home = if let UiState::Home = state.ui {
            style_text("home", true)
        } else {
            style_text("home", false)
        };
        frame.render_widget(home, chunks_main[0]);

        let search = if let UiState::Search = state.ui {
            style_text("search", true)
        } else {
            style_text("search", false)
        };
        frame.render_widget(search, chunks_main[1]);
    }

    pub fn handel_key_event(&mut self, key_event: KeyEvent, state: &mut State) {
        match key_event.code {
            KeyCode::Enter => state.focus.enter(),
            KeyCode::Char('j') => state.ui.next(),
            KeyCode::Char('k') => state.ui.prev(),
            KeyCode::Char('h') => state.focus.prev(),
            KeyCode::Char('l') => state.focus.next(),
            _ => {},
        }
    }
}
