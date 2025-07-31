use iced::Element;
use super::search_page::{SearchPage, SearchPageMessage};
use super::home_page::HomePage;
use super::super::state::State;

pub struct PageWidget {
    state: State,
    home_page: HomePage,
    search_page: SearchPage,
}

#[derive(Debug, Clone)]
pub enum PageWidgetMessage {
    StateChanged(State),
    HomePageMessage,
    SearchPageMessage(SearchPageMessage),
}

impl PageWidget {
    pub fn new() -> PageWidget {
        Self {
            state: State::default(),
            home_page: HomePage::new(),
            search_page: SearchPage::new(),
        }
    }

    pub fn update(&mut self, message: PageWidgetMessage) {
        match message {
            PageWidgetMessage::StateChanged(state) => {
                self.state = state;
            },
            PageWidgetMessage::HomePageMessage => {
                // Nothing
            },
            PageWidgetMessage::SearchPageMessage(message) => {
                self.search_page.update(message);
            },
        }
    }

    pub fn view(&self) -> Element<PageWidgetMessage> {
        let ret = match self.state {
            State::Home => {
                self.home_page
                    .view()
                    .map(|()| PageWidgetMessage::HomePageMessage)
            },
            State::Search => {
                self.search_page
                    .view()
                    .map(|msg| PageWidgetMessage::SearchPageMessage(msg))
            },
        };
        ret
    }
}
