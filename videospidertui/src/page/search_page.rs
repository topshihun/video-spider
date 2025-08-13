use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Position, Rect},
    style::{Color, Style},
    widgets::{Block, Paragraph},
};

use crate::state::{FocusState, PageState, State};

enum InputMod {
    Normal,
    Editing,
}

pub struct SearchPage {
    input: String,
    input_mod: InputMod,
    chracter_index: usize,
}

impl SearchPage {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            input_mod: InputMod::Normal,
            chracter_index: 0,
        }
    }

    pub fn draw(&self, frame: &mut Frame, area: Rect, _state: &PageState) {
        let chunks = Layout::vertical([Constraint::Length(3)])
            .margin(1)
            .split(area);

        // draw search input
        let search_input = Paragraph::new(self.input.as_str())
            .style(match self.input_mod {
                InputMod::Normal => Style::default().fg(Color::White),
                InputMod::Editing => Style::default().fg(Color::Yellow),
            })
            .block(Block::bordered().title("input"));
        frame.render_widget(search_input, chunks[0]);
        match self.input_mod {
            InputMod::Editing => {
                frame.set_cursor_position(Position::new(
                    chunks[0].x + self.chracter_index as u16 + 1,
                    chunks[0].y + 1,
                ));
            }
            _ => {}
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent, state: &mut FocusState) {
        match self.input_mod {
            InputMod::Normal => match key_event.code {
                KeyCode::Esc => state.escape(),
                KeyCode::Char('i') => self.input_mod = InputMod::Editing,
                _ => {}
            },
            InputMod::Editing => match key_event.code {
                KeyCode::Esc => self.input_mod = InputMod::Normal,
                KeyCode::Char(to_insert) => self.enter_char(to_insert),
                KeyCode::Backspace => self.delete_char(),
                KeyCode::Left => self.move_cursor_left(),
                KeyCode::Right => self.move_cursor_right(),
                _ => {}
            },
        }
    }

    fn start_search(&self) {
        todo!()
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.chracter_index.saturating_sub(1);
        self.chracter_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.chracter_index.saturating_add(1);
        self.chracter_index = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.chracter_index)
            .unwrap_or(self.input.len())
    }

    fn delete_char(&mut self) {
        let current_index = self.chracter_index;
        let from_left_to_current_index = current_index - 1;

        let befor_char_to_delete = self.input.chars().take(from_left_to_current_index);
        let after_char_to_delete = self.input.chars().skip(current_index);

        self.input = befor_char_to_delete.chain(after_char_to_delete).collect();
        self.move_cursor_left();
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }
}
