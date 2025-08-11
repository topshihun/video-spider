use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{layout::{Constraint, Layout}, DefaultTerminal, Frame};

use crate::{page::Page, series_tab::SeriesTab, state::{FocusState, State}, tab::Tab};

pub struct App {
    exit: bool,
    state: State,
    tab: Tab,
    series_tab: SeriesTab,
    page: Page,
}

impl App {
    pub fn new() -> Self {
        Self {
            exit: false,
            state: State::new(),
            tab: Tab::new(),
            series_tab: SeriesTab::new(),
            page: Page::new(),
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> std::io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let chunks = Layout::horizontal([
            Constraint::Length(30),
            Constraint::Min(30),
        ])
            .margin(1)
            .split(frame.area());

        let tab_chunks = Layout::vertical([
            Constraint::Length(4),
            Constraint::Min(2),
        ])
            .margin(0)
            .split(chunks[0]);

        self.tab.draw(frame, tab_chunks[0], &self.state);
        self.series_tab.draw(frame, tab_chunks[1], &self.state);
        self.page.draw(frame, chunks[1], &self.state);
    }


    fn handle_events(&mut self) -> std::io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            },
            _ => {},
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        if let KeyCode::Char('q') = key_event.code {
            return self.exit();
        }
        match self.state.focus {
            FocusState::Tab => self.tab.handel_key_event(key_event, &mut self.state),
            FocusState::SeriesTab => self.series_tab.handel_key_event(key_event, &mut self.state),
            FocusState::Page => self.page.handel_key_event(key_event, &mut self.state),
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

}
