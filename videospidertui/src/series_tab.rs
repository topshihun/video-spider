use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
};

use crate::{
    state::{FocusState, SeriesTabState, State},
    utils::{style_block, style_text},
};

pub struct SeriesTab {}

impl SeriesTab {
    pub fn new() -> Self {
        Self {}
    }

    pub fn draw(&self, frame: &mut Frame, area: Rect, state: (&SeriesTabState, &FocusState)) {
        let (series_tab_state, focus_state) = state;

        let focus = matches!(focus_state, FocusState::SeriesTab);

        let block = style_block("series", focus);
        let block_inner = block.inner(area);
        frame.render_widget(block, area);

        let series_index = series_tab_state.index.or(None);
        let mut text_list = Vec::new();
        for (index, series) in series_tab_state.series_list.iter().enumerate() {
            let text = if let Some(series_index) = series_index {
                if series_index == index {
                    style_text(series.name.as_str(), focus)
                } else {
                    style_text(series.name.as_str(), false)
                }
            } else {
                style_text(series.name.as_str(), false)
            };
            text_list.push(text);
        }
        let chunks =
            Layout::vertical(text_list.iter().map(|_| Constraint::Length(1))).split(block_inner);
        for (chunk, text) in chunks.iter().zip(text_list.iter()) {
            frame.render_widget(text, *chunk);
        }
    }

    pub fn handel_key_event(&mut self, key_event: KeyEvent, state: &mut State) {
        match key_event.code {
            KeyCode::Enter => state.focus_state.enter(),
            KeyCode::Char('j') => {
                state.series_tab_state.write().unwrap().next();
            }
            KeyCode::Char('k') => {
                state.series_tab_state.write().unwrap().prev();
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
