mod input;
mod lua_file_tab;

use std::collections::HashMap;
use std::sync::mpsc::{Sender, channel};
use std::sync::{Arc, RwLock};
use std::thread;

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::*;
use ratatui::widgets::{Block, List, ListState, Paragraph};
use ratatui::{
    Frame,
    layout::{Constraint, Layout, Rect},
};
use videospider::{LuaFile, SearchMessage, Series, get_lua_files, search};

use crate::message::Message;
use crate::page::search_page::lua_file_tab::LuaFileTab;
use crate::state::{FocusState, State};
use crate::{page::search_page::input::Input, state::PageState};

enum InputMod {
    Normal,
    Editing,
}

type SeriesList = Vec<Arc<Series>>;

pub struct SearchPage {
    sender: Sender<Message>,
    input: Input,
    lua_file_tab: LuaFileTab,
    input_mod: InputMod,
    list_state: ListState,
    series_list_map: Arc<RwLock<HashMap<Arc<LuaFile>, videospider::Result<SeriesList>>>>,
}

impl SearchPage {
    pub fn new(sender: Sender<Message>) -> Self {
        let lua_file_list = get_lua_files();
        let arc_lua_file_list: Vec<Arc<LuaFile>> =
            lua_file_list.into_iter().map(Arc::new).collect();
        let series_list_map = HashMap::with_capacity(arc_lua_file_list.len());

        Self {
            sender,
            input: Input::new(),
            lua_file_tab: LuaFileTab::new(arc_lua_file_list),
            input_mod: InputMod::Normal,
            list_state: ListState::default(),
            series_list_map: Arc::new(RwLock::new(series_list_map)),
        }
    }

    pub fn draw(&mut self, frame: &mut Frame, area: Rect, _state: &PageState) {
        let chunks = Layout::vertical([
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Min(3),
        ])
        .margin(1)
        .split(area);

        // draw search input
        self.input.draw(frame, chunks[0], &self.input_mod);

        // draw lua_file block
        self.lua_file_tab.draw(frame, chunks[1]);

        // draw search result block
        if let Some(lua_file) = self.lua_file_tab.get() {
            match self.series_list_map.read().unwrap().get(lua_file) {
                Some(result) => match result {
                    Ok(series_list) => {
                        let items: Vec<String> = series_list
                            .iter()
                            .map(|series| series.name.clone())
                            .collect();
                        let list = List::new(items)
                            .block(Block::bordered().title("search"))
                            .highlight_style(Style::new().reversed());
                        frame.render_stateful_widget(list, chunks[2], &mut self.list_state);
                    }
                    // TODO: show error information.
                    Err(_e) => {
                        let paragraph =
                            Paragraph::new("error").block(Block::bordered().title("errro"));
                        frame.render_widget(paragraph, chunks[2]);
                    }
                },
                None => {
                    let paragraph =
                        Paragraph::new("Nothing").block(Block::bordered().title("search"));
                    frame.render_widget(paragraph, chunks[2]);
                }
            }
        }
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent, state: &mut State) {
        match self.input_mod {
            InputMod::Normal => match key_event.code {
                KeyCode::Esc | KeyCode::Char('q') => state.focus_state.escape(&state.page_state),
                KeyCode::Char('i') => self.input_mod = InputMod::Editing,
                KeyCode::Char('j') => self.list_state.select_next(),
                KeyCode::Char('k') => self.list_state.select_previous(),
                KeyCode::Char('h') => self.lua_file_tab.prev(),
                KeyCode::Char('l') => self.lua_file_tab.next(),
                KeyCode::Enter => self.enter_series(state),
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
        if let Some(lua_file) = self.lua_file_tab.get() {
            let lua_file = lua_file.clone();
            let word = self.input.get().clone();
            let series_list_map = Arc::clone(&self.series_list_map);
            let sender_thread = self.sender.clone();
            thread::spawn(move || {
                let (sender, recv) = channel::<SearchMessage>();
                search(sender, &[lua_file], &word);
                while let Ok(search_result) = recv.recv() {
                    match search_result {
                        SearchMessage::Continue(lua_file, result) => {
                            let result = match result {
                                Ok(o) => {
                                    let arc_series_list = o.into_iter().map(Arc::new).collect();
                                    videospider::Result::Ok(arc_series_list)
                                }
                                Err(e) => videospider::Result::Err(e),
                            };
                            series_list_map
                                .write()
                                .unwrap()
                                .insert(Arc::new(lua_file), result);
                        }
                        SearchMessage::Finished => break,
                    }
                }
                sender_thread.send(Message::Update).unwrap();
            });
        }
    }

    fn enter_series(&self, state: &mut State) {
        if let Some(index) = self.list_state.selected() {
            // get series
            let lua_file = self.lua_file_tab.get().unwrap().clone();
            let series_list_map = self.series_list_map.read().unwrap();
            let series_list_result = series_list_map.get(&lua_file).unwrap();
            if let Ok(series_list) = series_list_result {
                // put series to tab widget
                let series = series_list.get(index).unwrap();
                state.series_tab_state.write().unwrap().push_series(series);
                state.focus_state = FocusState::SeriesTab;
                state.update_page_state();
            }
        }
    }
}
