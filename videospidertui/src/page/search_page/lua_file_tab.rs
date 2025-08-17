use std::sync::Arc;

use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use videospider::luafiles::LuaFile;

use crate::utils::style_block;

pub struct LuaFileTab {
    index: Option<usize>,
    lua_file_list: Vec<Arc<LuaFile>>,
}

impl LuaFileTab {
    pub fn new(lua_file_list: Vec<Arc<LuaFile>>) -> Self {
        Self {
            index: if !lua_file_list.is_empty() {
                Some(0)
            } else {
                None
            },
            lua_file_list,
        }
    }

    pub fn draw(&self, frame: &mut Frame, area: Rect) {
        let block = style_block("lua", false);
        let block_inner = block.inner(area);
        frame.render_widget(block, area);

        let text_list: Vec<Text> = self
            .lua_file_list
            .iter()
            .map(|lua_file| Text::from(lua_file.name.clone()))
            .collect();
        let mut constraint_list: Vec<Constraint> = Vec::with_capacity(text_list.len());
        for _ in 0..text_list.len() {
            constraint_list.push(Constraint::Fill(1));
        }
        let len = constraint_list.len();
        let chunks = Layout::horizontal(constraint_list)
            .spacing(1)
            .split(block_inner);
        for i in 0..len {
            if let Some(index) = self.index {
                let mut text = text_list.get(index).unwrap().clone();
                if i == index {
                    text = text_list
                        .get(i)
                        .unwrap()
                        .clone()
                        .style(Style::new().bg(Color::Blue));
                }
                frame.render_widget(text, chunks[i]);
            } else {
                frame.render_widget(text_list.get(i).unwrap(), chunks[i]);
            }
        }
    }

    pub fn next(&mut self) {
        if let Some(i) = self.index {
            if i == self.lua_file_list.len() - 1 {
                self.index = Some(0);
            } else {
                self.index = Some(i.saturating_add(1));
            }
        }
    }

    pub fn prev(&mut self) {
        if let Some(i) = self.index {
            if i == 0 {
                self.index = Some(self.lua_file_list.len() - 1);
            } else {
                self.index = Some(i.saturating_sub(1));
            }
        }
    }

    pub fn get(&self) -> Option<&LuaFile> {
        if let Some(index) = self.index {
            Some(self.lua_file_list.get(index).unwrap())
        } else {
            None
        }
    }
}
