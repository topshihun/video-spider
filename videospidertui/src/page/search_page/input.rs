use ratatui::layout::Position;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, Paragraph},
};

use crate::page::search_page::InputMod;

#[derive(Debug)]
pub struct Input {
    buff: String,
    chracter_index: usize,
}

impl Input {
    pub fn new() -> Self {
        Self {
            buff: String::new(),
            chracter_index: 0,
        }
    }

    pub fn draw(&self, frame: &mut Frame, area: Rect, state: &InputMod) {
        let search_input = Paragraph::new(self.buff.as_str())
            .style(match state {
                InputMod::Normal => Style::default().fg(Color::White),
                InputMod::Editing => Style::default().fg(Color::Yellow),
            })
            .block(Block::bordered().title("input"));
        frame.render_widget(search_input, area);

        if let InputMod::Editing = state {
            frame.set_cursor_position(Position::new(
                area.x + self.chracter_index as u16 + 1,
                area.y + 1,
            ));
        }
    }

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.chracter_index.saturating_sub(1);
        self.chracter_index = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.chracter_index.saturating_add(1);
        self.chracter_index = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.buff.insert(index, new_char);
        self.move_cursor_right();
    }

    fn byte_index(&self) -> usize {
        self.buff
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.chracter_index)
            .unwrap_or(self.buff.len())
    }

    pub fn delete_char(&mut self) {
        let current_index = self.chracter_index;
        let from_left_to_current_index = current_index - 1;

        let befor_char_to_delete = self.buff.chars().take(from_left_to_current_index);
        let after_char_to_delete = self.buff.chars().skip(current_index);

        self.buff = befor_char_to_delete.chain(after_char_to_delete).collect();
        self.move_cursor_left();
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.buff.chars().count())
    }
}
