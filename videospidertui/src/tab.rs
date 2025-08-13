use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{layout::{Constraint, Layout, Rect}, Frame};

use crate::{state::{FocusState, State, TabState}, utils::{style_block, style_text}};

pub struct Tab {
}

impl Tab {
    pub fn new() -> Tab {
        Self {
        }
    }

    pub fn draw(&self, frame: &mut Frame, area: Rect, state: (&TabState, &FocusState)) {
        let (tab_state, focus_state) = state;

        let block = if let FocusState::Tab = focus_state {
            style_block("tab", true)
        } else {
            style_block("tab", false)
        };
        frame.render_widget(block, area);

        let chunks_main = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(1),
        ])
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

    pub fn handel_key_event(&mut self, key_event: KeyEvent, state: (&mut TabState, &mut FocusState)) {
        let (tab_state, focus_state) = state;

        match key_event.code {
            KeyCode::Enter => focus_state.enter(),
            KeyCode::Char('j') => tab_state.next(),
            KeyCode::Char('k') => tab_state.prev(),
            KeyCode::Char('h') => focus_state.prev(),
            KeyCode::Char('l') => focus_state.next(),
            _ => {},
        }
    }
}
