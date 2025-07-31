use iced::Element;
use super::search_page::{ SearchPage, SearchPageMessage};

pub struct PageWidget {
    search_page: SearchPage,
}

#[derive(Debug, Clone)]
pub enum PageWidgetMessage {
    SearchPageMessage(SearchPageMessage),
}

impl PageWidget {
    pub fn new() -> PageWidget {
        Self {
            search_page: SearchPage::new(),
        }
    }

    pub fn update(&mut self, message: PageWidgetMessage) {
        match message {
            PageWidgetMessage::SearchPageMessage(message) => self.search_page.update(message),
        }
    }

    pub fn view(&self) -> Element<PageWidgetMessage> {
        self.search_page
            .view()
            .map(|msg| PageWidgetMessage::SearchPageMessage(msg))
    }
}
