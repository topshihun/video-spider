use std::sync::mpsc::{Receiver, Sender};

use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{Constraint, Layout},
};

use crate::{
    message::Message,
    page::Page,
    series_tab::SeriesTab,
    state::{FocusState, State},
    tab::Tab,
};

pub struct App {
    state: State,
    tab: Tab,
    series_tab: SeriesTab,
    page: Page,
}

impl App {
    pub fn new(sender: Sender<Message>) -> Self {
        Self {
            state: State::new(),
            tab: Tab::new(),
            series_tab: SeriesTab::new(),
            page: Page::new(sender),
        }
    }

    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
        recv: Receiver<Message>,
    ) -> std::io::Result<()> {
        terminal.draw(|frame| self.draw(frame))?;
        while !self.state.exit {
            match recv.recv().expect("app run receive error") {
                Message::Update => {
                    terminal.draw(|frame| self.draw(frame))?;
                }
                Message::KeyEvent(key_event) => {
                    self.handle_key_event(key_event);
                    terminal.draw(|frame| self.draw(frame))?;
                }
            }
        }
        Ok(())
    }

    fn draw(&mut self, frame: &mut Frame) {
        let chunks = Layout::horizontal([Constraint::Length(30), Constraint::Min(30)])
            .margin(1)
            .split(frame.area());

        let tab_chunks = Layout::vertical([Constraint::Length(4), Constraint::Min(2)])
            .margin(0)
            .split(chunks[0]);

        self.tab.draw(
            frame,
            tab_chunks[0],
            (
                &self.state.tab_state.read().unwrap(),
                &self.state.focus_state,
            ),
        );
        self.series_tab.draw(
            frame,
            tab_chunks[1],
            (
                &self.state.series_tab_state.read().unwrap(),
                &self.state.focus_state,
            ),
        );
        self.page.draw(
            frame,
            chunks[1],
            (&self.state.page_state, &self.state.focus_state),
        );
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match self.state.focus_state {
            FocusState::Tab => {
                if let KeyCode::Char('q') = key_event.code {
                    self.state.exit = true;
                }
                self.tab.handel_key_event(key_event, &mut self.state);
            }
            FocusState::SeriesTab => {
                if let KeyCode::Char('q') = key_event.code {
                    self.state.exit = true;
                }
                self.series_tab.handel_key_event(key_event, &mut self.state);
            }
            FocusState::Page => self.page.handel_key_event(key_event, &mut self.state),
        }
    }
}
