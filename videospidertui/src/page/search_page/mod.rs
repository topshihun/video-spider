mod input;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::*;
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Position, Rect},
    style::{Color, Style},
    text::{Line, Masked, Span},
    widgets::{Block, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState},
};

use crate::{
    page::search_page::input::Input,
    state::{FocusState, PageState, State},
    utils::style_block,
};

enum InputMod {
    Normal,
    Editing,
}

pub struct SearchPage {
    input: Input,
    input_mod: InputMod,
    scroll_state: ScrollbarState,
    scroll: usize,
}

impl SearchPage {
    pub fn new() -> Self {
        Self {
            input: Input::new(),
            input_mod: InputMod::Normal,
            scroll_state: ScrollbarState::default(),
            scroll: 0,
        }
    }

    pub fn draw(&self, frame: &mut Frame, area: Rect, _state: &PageState) {
        let chunks = Layout::vertical([Constraint::Length(3), Constraint::Min(3)])
            .margin(1)
            .split(area);

        // draw search input
        self.input.draw(frame, chunks[0], &self.input_mod);

        let text = vec![];
        // draw search result block
        let result_paragraph = Paragraph::new(text.clone())
            .gray()
            .block(style_block("search", false))
            .scroll((self.scroll as u16, 0));
        frame.render_widget(result_paragraph, chunks[1]);

        let mut scroll_state = self.scroll_state.content_length(text.len());
        let scroll_bar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));
        frame.render_stateful_widget(scroll_bar, chunks[1], &mut scroll_state);
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent, state: &mut FocusState) {
        match self.input_mod {
            InputMod::Normal => match key_event.code {
                KeyCode::Esc => state.escape(),
                KeyCode::Char('i') => self.input_mod = InputMod::Editing,
                KeyCode::Char('j') => self.scroll = self.scroll.saturating_add(1),
                KeyCode::Char('k') => self.scroll = self.scroll.saturating_sub(1),
                _ => {}
            },
            InputMod::Editing => match key_event.code {
                KeyCode::Esc => self.input_mod = InputMod::Normal,
                KeyCode::Char(to_insert) => self.input.enter_char(to_insert),
                KeyCode::Backspace => self.input.delete_char(),
                KeyCode::Left => self.input.move_cursor_left(),
                KeyCode::Right => self.input.move_cursor_right(),
                KeyCode::Enter => self.start_search(),
                _ => {}
            },
        }
    }

    fn start_search(&self) {
        todo!()
    }
}
