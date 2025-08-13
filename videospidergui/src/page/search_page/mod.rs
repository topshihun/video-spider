mod series_list_widget;
mod series_widget;

use iced::Element;
use iced::widget::{button, row, text_input};

pub struct SearchPage {
    content: String,
}

#[derive(Debug, Clone)]
pub enum SearchPageMessage {
    SearchInputChanged(String),
    SearchButtonClicked,
}

impl SearchPage {
    pub fn new() -> Self {
        Self {
            content: String::from(""),
        }
    }

    pub fn update(&mut self, message: SearchPageMessage) {
        match message {
            SearchPageMessage::SearchInputChanged(str) => {
                self.content = str;
            }
            SearchPageMessage::SearchButtonClicked => {
                todo!("search action")
            }
        }
    }

    pub fn view(&self) -> Element<SearchPageMessage> {
        row![
            text_input("search input", &self.content)
                .on_input(SearchPageMessage::SearchInputChanged),
            button("search").on_press(SearchPageMessage::SearchButtonClicked),
        ]
        .into()
    }
}
